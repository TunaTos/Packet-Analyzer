use std::fmt::format;
use pnet::datalink;
use pnet::datalink::NetworkInterface;


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

pub fn show_running_interfaces() {
    println!("실행중인 interface를 출력합니다");

    let interfaces = datalink::interfaces();

    interfaces.iter().for_each(|interface| if interface.is_running() {
        print_interface(interface);
    });
}

pub fn show_all_interfaces() {
    let interfaces = datalink::interfaces();

    for interface in interfaces {

    }
}