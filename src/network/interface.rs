use pnet::datalink;
use pnet::datalink::NetworkInterface;

/**
1. 모든 interface 출력
2. running 중인 interface 출력
3. loop_back interface 출력
4. WIFI/Ethernet Interface (en*, et*, wlan*)
5. VPN/Tunnel interface 출력 (utun*, tun*, tap*)
*/

pub fn show_interfaces() {
    println!("{}", "=".repeat(60));
    println!("option1 : 모든 인터페이스 출력");
    println!("option2 : 실행중인 인터페이스 출력");
    println!("option3 : loob_back 인터페이스 출력");
    println!("option4 : WIFI/Ethernet Interface 출력");
    println!("option5 : VPN/Tunnel interface 출력");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let input: Option<u32> = buf.trim().parse().ok();

    match input {
        Some(1) => {
            show_all_interfaces();
        }
        Some(2) => {
            show_running_interfaces();
        }
        Some(3) => {
            show_loopback_interfaces();
        }
        Some(4) => {
            show_physical_interfaces();
        }
        Some(5) => {
            show_vpn_interfaces();
        }
        Some(_) => {
            return;
        }
        None => {
            println!("부정확한 입력을 진행했습니다");
        }
    }
}

pub fn print_interface(interface: &NetworkInterface) {
    println!("[{}]", interface.name);

    if interface.ips.is_empty() {
        println!("  no IP");
    } else {
        for ip in interface.ips.iter() {
            if ip.is_ipv4() {
                print!("  IPv4  ");
            } else if ip.is_ipv6() {
                print!("  IPv6  ");
            }
            println!("  {}/{}", ip.ip(), ip.prefix());
        }
    }
}

pub fn show_all_interfaces() {
    let interfaces = datalink::interfaces();

    interfaces
        .iter()
        .for_each(|interface| print_interface(interface));
}

pub fn show_running_interfaces() {
    println!("실행중인 interface를 출력합니다");

    let interfaces = datalink::interfaces();
    interfaces
        .iter()
        .filter(|interface| interface.is_running())
        .for_each(|interface| print_interface(&interface));
}

pub fn show_loopback_interfaces() {
    let interfaces = datalink::interfaces();

    interfaces
        .iter()
        .filter(|interface| interface.is_loopback())
        .for_each(|interface| print_interface(interface));
}

pub fn show_physical_interfaces() {
    let interfaces = datalink::interfaces();

    interfaces
        .iter()
        .filter(|interface| {
            interface.name.starts_with("en")
                || interface.name.starts_with("eth")
                || interface.name.starts_with("wlan")
                || interface.name.starts_with("wl")
        })
        .for_each(|interface| print_interface(&interface));
}

pub fn show_vpn_interfaces() {
    let interfaces = datalink::interfaces();

    interfaces
        .iter()
        .filter(|interface| {
            interface.name.starts_with("utun")
                || interface.name.starts_with("tun")
                || interface.name.starts_with("tap")
                || interface.name.starts_with("ppp")
        })
        .for_each(|interface| print_interface(&interface));
}
