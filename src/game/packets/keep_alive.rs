use crate::game::enums;
use crate::models::client;
use crate::net;

/// ## ?
/// This packet must be sent periodically,
/// otherwise the server will disconnect
/// the client after few seconds.
pub struct KeepAlive {
    pub enum_type: enums::Packet,
}

impl KeepAlive {
    pub fn new() -> Self {
        KeepAlive {
            enum_type: enums::Packet::KEEP_ALIVE,
        }
    }
    pub fn write(&self, bot: &mut client::Bot) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);

        b_arr
            .write_byte(self.enum_type as u8)
            .write_int(bot.net.cr2_token1.unwrap_or_default())
            .write_int(bot.net.cr2_token2.unwrap_or_default())
            .write_int(0xFCF869AC)
            .write_int(bot.net.rng_token1);

        return b_arr.data;
    }
}
