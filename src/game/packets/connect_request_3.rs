use crate::game::enums;
use crate::game::rng_crypt;
use crate::models::client;
use crate::net;
use rand::Rng;

/// ## ?
/// Used to establish a session
/// between the client and the server.
/// Ref: Game's home screen.
pub struct ConnectRequest3 {
    pub enum_type: enums::Packet,
    game_mode: enums::GameMode,
    no_public_first_connection: bool,
    mayhem_ticked: bool,
}

impl ConnectRequest3 {
    pub fn new(
        game_mode: enums::GameMode,
        no_public_first_connection: bool,
        mayhem_ticked: bool,
    ) -> Self {
        ConnectRequest3 {
            enum_type: enums::Packet::CONNECT_REQUEST_3,
            game_mode,
            no_public_first_connection,
            mayhem_ticked,
        }
    }

    /// ## Encryption?
    /// > The client weakly "encrypts" the packet,
    ///   probably to stop sniffers from retreiving data
    ///   more easily. However, this is easily bypassable
    ///   by just obtaining the "decription" key,
    ///   located in `.write_long(rng_crypt_key)`.
    ///   De-/encryption is filed in `./game/rng_crypt.rs`
    pub fn write(&self, bot: &mut client::Bot) -> Vec<u8> {
        let rng_crypt_key = rand::thread_rng().gen::<u64>();
        let mut b_arr = net::ByteArray::new(None);

        b_arr
            .write_byte(self.enum_type as u8)
            .write_int(0x00000000)
            .write_long(rng_crypt_key)
            .write_short(0x04A1)
            .write_int(bot.net.rng_token1)
            .write_byte(self.game_mode as u8)
            .write_byte(self.no_public_first_connection as u8)
            .write_int(0xFFFFFFFF)
            .write_utf8(bot.player_data.ticket)
            .write_byte(bot.player_data.visibility.unwrap_or_default() as u8)
            .write_bool(self.mayhem_ticked)
            .write_short(bot.player_data.skin.unwrap_or_default() as u16)
            .write_byte(bot.player_data.eject_skin.unwrap_or(0xFF) as u8)
            .write_utf8(&bot.player_data.name)
            .write_int(0x00000000)
            .write_byte(bot.player_data.name.len() as u8)
            .write_raw(&vec![0xFF; bot.player_data.name.len()])
            .write_byte(0xFF)
            .write_int(0xFF828282)
            .write_utf8("")
            .write_byte(bot.player_data.hat.unwrap_or(0xFF) as u8)
            .write_int(0x00000000)
            .write_byte(bot.player_data.halo.unwrap_or(0x00))
            .write_byte(0xFF)
            .write_utf8("")
            .write_int(0x00000000)
            .write_int(0x00000000)
            .write_byte(bot.player_data.particle.unwrap_or(0xFF) as u8)
            .write_byte(bot.player_data.name_font.unwrap_or_default() as u8)
            .write_byte(0x05)
            .write_raw(&vec![0x05; 0x77])
            .write_byte(bot.player_data.rainbow_cycle.unwrap_or_default() as u8)
            .write_short(bot.player_data.skin.unwrap_or_default() as u16)
            .write_short(0x0000)
            .write_int(0x00000000)
            .write_long(chrono::Utc::now().timestamp_millis() as u64);

        b_arr = rng_crypt::encrypt(b_arr, rng_crypt_key);

        return b_arr.data;
    }
}
