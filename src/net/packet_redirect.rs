use crate::game::packets;
use crate::models;
use crate::print;

pub fn redirect(bot: &mut models::Bot, packet: Vec<u8>) {
    let packet_id = packet[0];

    match packet_id {
        0x01 => {
            let mut handler = packets::ConnectResult2::new(packet);
            handler.parse(bot);
        }
        _ => print::wtf("Undefined packet", &format!("Packet ID: {}", packet_id)),
    }
}
