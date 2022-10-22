use crate::game::enums;
use crate::models;
use crate::net;

/// ## ?
/// This packet must be sent periodically,
/// otherwise the server will disconnect
/// the client after few seconds.
pub struct KeepAlive<'a> {
    pub enum_type: enums::Packet,
    bot: &'a mut models::Bot<'a>,
}

impl<'a> KeepAlive<'a> {
    pub fn new(bot: &'a mut models::Bot<'a>) -> Self {
        KeepAlive {
            enum_type: enums::Packet::KEEP_ALIVE,
            bot,
        }
    }
    pub fn write(&self) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);

        b_arr
            .write_byte(self.enum_type as u8)
            .write_int(self.bot.net.cr2_token1.unwrap_or_default())
            .write_int(self.bot.net.cr2_token2.unwrap_or_default())
            .write_int(0xFCF869AC)
            .write_int(self.bot.net.rng_token1);

        return b_arr.data;
    }
}
