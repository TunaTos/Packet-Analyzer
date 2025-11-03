use crate::network::interface::{show_all_interfaces, show_interfaces, show_running_interfaces};
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

pub fn start() {
    loop {
        display_menu();

        match handle_input() {
            Some(1) => {
                show_interfaces();
            }
            Some(2) => {
                println!("패킷 캡처를 시작합니다");
                // TODO!
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
