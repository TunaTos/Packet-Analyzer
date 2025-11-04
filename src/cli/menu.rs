use crate::network::capture::start_capture;
use crate::network::interface::*;
use std::io;
use std::io::stdin;

pub fn display_menu() {
    println!("\n=================메뉴=================");
    println!("1. 네트워크 인터페이스 확인.");
    println!("2. 네트워크 인터페이스 packet capture 시작하기");
    println!("3. 종료");
    println!("====================================");
}

pub fn handle_input() -> Option<usize> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok()?;
    buf.trim().parse().ok()
}

pub fn start_capture_menu() {
    println!("\n사용 가능한 인터페이스:");
    show_all_interfaces();

    println!("\n캡처할 인터페이스 이름을 입력하세요!");
    let mut interface_name = String::new();
    io::stdin().read_line(&mut interface_name).unwrap();
    let interface_name = interface_name.trim();

    if (interface_name.is_empty()) {
        println!("인터페이 이름이 비어있습니다");
        return;
    }
    start_capture(&interface_name);
}

pub fn start() {
    loop {
        display_menu();

        match handle_input() {
            Some(1) => {
                show_interfaces();
            }
            Some(2) => {
                start_capture_menu();
            }
            Some(3) => {
                println!("종료를 합니다");
                break;
            }
            Some(_) => {
                println!("잘못된 입력입니다");
            }
            None => {
                println!("입력 오류입니다! 숫자를 입력하세요!");
            }
        }
    }
}
