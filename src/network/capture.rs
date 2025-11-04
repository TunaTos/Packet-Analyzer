use crate::network::interface::find_interface_by_name;
use chrono::Local;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket};
use pnet::packet::{Packet, PacketSize};

pub fn start_capture(interface_name: &str) {
    let interface = find_interface_by_name(interface_name).unwrap();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();

                println!(
                    "[{}] Ethernet: {} -> {} | Type: {:?} | Len: {} bytes",
                    Local::now().format("%H:%M:%S%.3f"),
                    packet.get_source(),
                    packet.get_destination(),
                    packet.get_ethertype(),
                    packet.packet_size()
                );
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
