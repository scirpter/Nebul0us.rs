use crate::game::enums;
use crate::models;

struct Packet {
    bot: models::Bot,
    packet_id: enums::PacketId,
    data: Vec<u8>,
}

impl Packet {

}
