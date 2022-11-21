use crate::game::enums;
use crate::models::client;
use crate::net;

/// ## ?
/// Read or write a message to
/// the public chat.
pub struct GameChatMessage<'a> {
    pub enum_type: enums::Packet,
    bot: &'a mut client::Bot<'a>,
    message: Option<&'a str>,
}

impl<'a> GameChatMessage<'a> {
    pub fn new(bot: &'a mut client::Bot<'a>, message: Option<&'a str>) -> Self {
        GameChatMessage {
            enum_type: enums::Packet::GAME_CHAT_MESSAGE,
            bot,
            message: message,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);
        b_arr
            .write_byte(self.enum_type as u8)
            .write_int(self.bot.net.rng_token1)
            .write_utf8(&self.bot.player_data.name)
            .write_utf8(self.message.unwrap_or("Weebs"))
            .write_int(0xFFFFFFFF)
            .write_long(0x0000000000000000)
            .write_short(self.bot.player_data.name.len() as u16)
            .write_raw(&vec![0xFF; self.bot.player_data.name.len()])
            .write_short(0x0000)
            .write_int(self.bot.net.rng_token1);
        return b_arr.data;
    }

    pub async fn parse(&mut self, data: Vec<u8>) {
        let mut b_arr = net::ByteArray::new(Some(data));
        let packet_id = b_arr.read_byte();

        let player_net_id = b_arr.read_int();
        let player_name = b_arr.read_utf8();
        if player_name == self.bot.player_data.name {
            return;
        }
        let message = b_arr.read_utf8();
        let player_id = b_arr.read_int();
        b_arr.read_bool();
        b_arr.read_long();

        // TODO: Add event listener
    }
}
