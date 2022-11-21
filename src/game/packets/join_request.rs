use crate::game::enums;
use crate::models::client;
use crate::net;

/// ## ?
/// When we have joined a game and
/// want the bots to start playing.
pub struct JoinRequest<'a> {
    pub enum_type: enums::Packet,
    bot: &'a mut client::Bot<'a>,
}

impl<'a> JoinRequest<'a> {
    pub fn new(bot: &'a mut client::Bot<'a>) -> Self {
        JoinRequest {
            enum_type: enums::Packet::JOIN_REQUEST,
            bot,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);
        b_arr
            .write_byte(self.enum_type as u8)
            .write_short(self.bot.player_data.skin.unwrap_or_default() as u16)
            .write_utf8(&self.bot.player_data.name)
            .write_short(0xFF00)
            .write_int(self.bot.player_data.name.len() as u32)
            .write_short(0xFFFF)
            .write_raw(&vec![0xFF; self.bot.player_data.name.len()])
            .write_hex("e1d452")
            .write_utf8("")
            .write_byte(self.bot.player_data.hat.unwrap_or(0xFF))
            .write_int(0x00000000)
            .write_byte(self.bot.player_data.halo.unwrap_or(0x00) as u8) // halo
            .write_byte(0xFF)
            .write_utf8("")
            .write_int(0x00000000)
            .write_int(0x00000000)
            .write_byte(self.bot.player_data.particle.unwrap_or(0xFF))
            .write_byte(self.bot.player_data.name_font.unwrap_or_default() as u8)
            .write_byte(0x05)
            .write_byte(self.bot.player_data.rainbow_cycle.unwrap_or_default() as u8)
            .write_short(0x0000)
            .write_int(0x00000000)
            .write_short(0x0000)
            .write_int(0x00000000)
            .write_int(self.bot.net.rng_token1)
            .write_hex("7777777777");
        return b_arr.data;
    }
}
