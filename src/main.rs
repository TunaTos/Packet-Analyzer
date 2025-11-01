use std::io::{self, Write};
use pnet::{datalink::{self, NetworkInterface}, packet};


fn main() {
    println!("=== 사용 가능한 네트워크 인터페이스 ===n");

    let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();

    if interfaces.is_empty() {
        println!("네트워크 인터페이스를 찾을 수 없습니다");
        return ;
    }

    for (index, interface)  in interfaces.iter().enumerate() {
        println!("[{}] {} (UP: {}, Loopback: {}",
            index,
            interface.name,
            interface.is_up(),
            interface.is_loopback()    
        );

        if !interface.ips.is_empty() {
            println!("          IP: {:?}", interface.ips.first());
        }
    }    

    println!("\n 캡처할 인터페이스 번호를 입력하세요!");
    println!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let selected_index: usize = input.trim().parse().expect("올바른 숫자를 입력하세요");

    if selected_index >= interfaces.len() {
        println!("잘못된 인터페이스 번호다");
        return ;
    }
    
    let selected_interface = &interfaces[selected_index];
    println!("\n[{}] 인터페이스에서 캡처를 시작합니다...", selected_interface.name);
    
    start_capture(selected_interface);

}


fn start_capture(interface: &NetworkInterface) {
    use pnet::datalink::Channel::Ethernet;

    // 채널 생성 (패킷을 읽고 쓸 수 있는 통로)
    let (mut tx, mut rx) = match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            println!("지원하지 않는 채널 타입입니다");
            return;
        }
        Err(e) => {
            println!("채널 생성 실패: {}", e);
            println!("힌트: sudo 권한이 필요할 수 있습니다");
            return;
        }
    };

    println!("패킷 캡처 중... (Ctrl+C로 종료)\n");

    let mut packet_count = 0;

    // 패킷 수신 루프
    loop {
        match rx.next() {
            Ok(packet) => {
                packet_count += 1;
                println!("[패킷 #{}] 크기: {} bytes", packet_count, packet.len());
                
                // 처음 20바이트만 16진수로 출력 (헤더 미리보기)
                print!("  데이터: ");
                for byte in packet.iter().take(20) {
                    print!("{:02x} ", byte);
                }
                println!("...\n");

                // 너무 빠르면 보기 힘드니 잠깐 멈춤
                if packet_count >= 10 {
                    println!("10개 패킷 캡처 완료! 계속하려면 더 많이 보고 싶으면 코드를 수정하세요.");
                    break;
                }
            }
            Err(e) => {
                println!("패킷 읽기 오류: {}", e);
                break;
            }
        }
    }

    println!("\n총 {} 개의 패킷을 캡처했습니다.", packet_count);
}
