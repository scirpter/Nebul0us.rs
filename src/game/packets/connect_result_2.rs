use crate::game::enums;
use crate::models::client;
use crate::net;

/// ## ?
/// Received when trying to establish
/// a connection to the server.
/// This holds important information like
/// `cr2_token1` and `cr2_token2`.
/// Those tokens are used to identify the
/// client server-side.
pub struct ConnectResult2 {
    pub enum_type: enums::Packet,
    data: Vec<u8>,
}

impl ConnectResult2 {
    pub fn new(data: Vec<u8>) -> Self {
        ConnectResult2 {
            enum_type: enums::Packet::CONNECT_RESULT_2,
            data,
        }
    }

    pub async fn parse<'b>(&mut self, bot: &mut client::Bot<'b>) {
        let data = self.data.clone();
        let mut b_arr = net::ByteArray::new(Some(data));

        let packet_id = b_arr.read_byte();
        let untrue_result = b_arr.read_byte(); // should actually return `enums::ConnectResult`, but it doesn't (random value instead)
        let e = b_arr.read_int();
        bot.net.cr2_token1 = Some(b_arr.read_int());
        bot.net.cr2_token2 = Some(b_arr.read_int());
        let g = b_arr.read_int();
        let read_float = b_arr.read_float();

        // aka doesn't care about illegal connect request
        bot.net.connection_state = Some(enums::ConnectionState::CONNECTED);
    }
}
