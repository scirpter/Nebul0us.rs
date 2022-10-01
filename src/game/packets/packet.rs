use crate::game::enums;
use crate::models;

trait PacketHandler {
    /// Append the packet ID to the packet
    fn write(&self, packet: Packet);
    /// Extract information from the packet
    fn parse(&self, packet: Packet);
}

pub struct Packet {
    bot: models::Bot,
    packet_id: enums::PacketId,
    data: Vec<u8>,
}

impl PacketHandler for Packet {
    fn write(&self, mut packet: Packet) {
        packet.data.insert(0, packet.packet_id as u8);
    }
    fn parse(&self, packet: Packet) {}
}

impl Packet {
    pub fn new(bot: models::Bot, packet_id: enums::PacketId, data: Option<Vec<u8>>) -> Packet {
        Packet {
            bot,
            packet_id,
            data: match data {
                Some(data) => data,
                None => Vec::new(),
            },
        }
    }
}
