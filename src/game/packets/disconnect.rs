use crate::game::enums;
use crate::models;
use crate::net;

/// ## ?
/// If a client session is running,
/// send this to disconnect.
/// Requires the client to reconnect.
pub struct Disconnect {
    pub enum_type: enums::Packet,
}

impl Disconnect {
    pub fn new() -> Self {
        Disconnect {
            enum_type: enums::Packet::DISCONNECT,
        }
    }

    pub fn write(&self, bot: &mut models::Bot) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);
        b_arr
            .write_byte(self.enum_type as u8)
            .write_int(bot.net.cr2_token1.unwrap_or_default())
            .write_int(bot.net.cr2_token2.unwrap_or_default())
            .write_int(bot.net.rng_token1);
        return b_arr.data;
    }
}
