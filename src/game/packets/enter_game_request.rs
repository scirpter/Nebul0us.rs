use crate::game::enums;

use crate::models::client;
use crate::net;

/// ## ?
/// Used to send the client
/// into a game.
pub struct EnterGameRequest<'a> {
    pub player_id: Option<u32>,
    // NOTE: is this unsafe to use? (strref)
    pub room_name: Option<&'a str>,
}

impl<'a> EnterGameRequest<'a> {
    pub fn new(player_id: Option<u32>, room_name: Option<&'a str>) -> Self {
        EnterGameRequest {
            player_id,
            room_name,
        }
    }

    pub fn write(&self, bot: &mut client::Bot) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);
        b_arr
            .write_byte(enums::Packet::ENTER_GAME_REQUEST as u8)
            .write_int(bot.net.cr2_token1.unwrap_or_default())
            .write_int(bot.net.rng_token1)
            .write_int(0xFFFFFFFF)
            .write_utf8(self.room_name.unwrap_or_default())
            .write_int(self.player_id.unwrap_or(0xFFFFFFFF))
            .write_byte(0xFF);
        return b_arr.data;
    }
}
