use crate::game::enums;
use crate::models::client;
use crate::net;

/// ## ?
/// Used to let the client
/// send an emote request,
/// while playing of course.
pub struct EmoteRequest {
    pub emote_id: u8,
}

impl EmoteRequest {
    pub fn new(emote_id: u8) -> Self {
        EmoteRequest { emote_id }
    }

    pub fn write(&self, bot: &mut client::Bot) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);
        b_arr
            .write_byte(enums::Packet::EMOTE_REQUEST as u8)
            .write_byte(self.emote_id)
            .write_int(bot.net.rng_token1)
            .write_int(0x00000000);
        return b_arr.data;
    }
}
