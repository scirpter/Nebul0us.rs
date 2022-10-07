use crate::game::enums;
use crate::{models, net};
use chrono;
use java_rand::Random as JavaRNG;
use rand::Rng;

use super::rng_crypt;

pub struct ConnectResult2 {}

pub struct Control {}

pub struct KeepAlive {}

pub struct InvalidateClient {}

pub struct StartGameInternal {}

/// ## DEPRECATED SINCE 6.0.3 (use `ConnectRequest3` instead)
pub struct ConnectRequest {}

/// ## ?
/// If a client session is running,
/// send this to disconnect.
/// Requires the client to reconnect.
pub struct Disconnect<'a> {
    enum_type: enums::Packet,
    bot: models::Bot<'a>,
}

impl<'a> Disconnect<'a> {
    pub fn new(bot: models::Bot) -> Disconnect {
        Disconnect {
            enum_type: enums::Packet::DISCONNECT,
            bot,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut b_arr = net::ByteArray::new(None);
        b_arr.write_byte(self.enum_type as u8);
        b_arr.write_int(self.bot.net.cr2_token1.unwrap_or_default());
        b_arr.write_int(self.bot.net.cr2_token2.unwrap_or_default());
        b_arr.write_int(self.bot.net.rng_token1);
        return b_arr.data;
    }
}

pub struct GameChatMessage {}

pub struct ClanChatMessage {}

/// ## ?
/// When we have joined a game and
/// want the bots to start playing.
pub struct JoinRequest<'a> {
    enum_type: enums::Packet,
    bot: models::Bot<'a>,
}

impl<'a> JoinRequest<'a> {
    pub fn new(bot: models::Bot) -> JoinRequest {
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
            .write_utf8(self.bot.player_data.name)
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

/// ## ?
/// When a player rejoins,
/// this is the packet received.
/// Excludes self.
pub struct JoinResult {}

pub struct TTLRefreshResponseInternal {}

pub struct ShutdownNodeInternal {}

pub struct SETGSAddr {}

pub struct ClientPreferences {}

pub struct SpectateChange {}

pub struct ClanWarListRequest {}

pub struct ClanWarListResult {}

pub struct ClanWarNotification {}

pub struct TopScores {}

pub struct ServerShutdownWarning {}

pub struct GameUpdate {}

pub struct GroupLobbyListRequest {}

pub struct GroupLobbyListResult {}

pub struct PublicChatMessage {}

pub struct AdminInternal {}

pub struct GroupLobbyCreateRequest {}

pub struct GroupLobbyCreateResult {}

pub struct GroupLobbyJoinRequest {}

pub struct GroupLobbyJoinResult {}

pub struct GroupLobbyUpdate {}

pub struct GroupLobbyLeave {}

pub struct ArenaListRequest {}

pub struct ClientPreferencesInternal {}

pub struct GameCrashInternal {}

pub struct PrivateChatMessage {}

pub struct ArenaLeaveQueueRequest {}

pub struct RemoveGameInternal {}

pub struct GroupLobbyWarn {}

pub struct EnterGameRequest {}

pub struct EnterGameResult {}

pub struct PlayerSessionStatsUpdateInternal {}

pub struct PlayerWSAccountUpdateInternal {}

pub struct AccountStatusRequest {}

pub struct AccountStatusResult {}

pub struct FriendChatMessage {}

pub struct ClientStatusChangeRequest {}

pub struct ClientStatusChangeResult {}

pub struct ClanWarControl {}

pub struct ClanWarUpdate {}

pub struct ArenaListResult {}

pub struct AdminInternal2 {}

pub struct NodeResetRequestInternal {}

pub struct ClanWarResultInternal {}

pub struct ClanWarForfeitInternal {}

pub struct SpectateGameRequest {}

pub struct GetPlayerStatsInternal {}

pub struct ArenaQueueRequest {}

pub struct ArenaStatus {}

pub struct AdminInternal3 {}

pub struct ArenaResultInternal {}

pub struct AdminInternal4 {}

pub struct TeamArenaResultInternal {}

pub struct TeamArenaStatusResult {}

pub struct TeamArenaStatusRequest {}

pub struct TeamArenaListRequest {}

pub struct TeamArenaListResult {}

pub struct TeamArenaQueueRequest {}

pub struct TeamArenaLeaveQueueReqeust {}

pub struct TeamArenaUpdate {}

pub struct ClanHouseUpdateInternal {}

pub struct AdminInternal5 {}

pub struct ClanHouseUpdateInternal2 {}

pub struct NodeConnectRequestInternal {}

pub struct GameData {}

pub struct Challenge {}

pub struct ChallengeResult {}

pub struct FwdToClientInternal {}

pub struct TTLRefreshRequestInternal {}

pub struct ConnectRequest2 {}

pub struct ConnectResult {}

pub struct AdminInternal6 {}

pub struct ClanHouseUpdateInternal3 {}

pub struct TourneyListRequest {}

pub struct TourneyListResult {}

pub struct TourneyAction {}

pub struct TourneyMatchResultInternal {}

pub struct TourneyStartInternal {}

pub struct TourneyStatusUpdate {}

pub struct AdminInternal7 {}

pub struct MuteInternal {}

pub struct JoinedGameInternal {}

pub struct ClanHouseUpdateInternal4 {}

pub struct ClanHouseConfig {}

pub struct Invite {}

pub struct DesiredDuoPartner {}

pub struct EmoteRequest {}

pub struct UDPKeepalive {}

pub struct GroupChatCreateRequest {}

pub struct GroupChatJoinRequest {}

pub struct GroupChatLeaveRequest {}

pub struct GroupChatResult {}

pub struct GroupChatStatus {}

pub struct GroupChatMessage {}

/// ## ?
/// When the bot dies or a round ends,
/// this packet is received.
/// This is more used to determine whether
/// the bot has died.
pub struct SessionStats<'a> {
    bot: models::Bot<'a>,
    data: Vec<u8>,
}

impl<'a> SessionStats<'a> {
    pub fn new(bot: models::Bot, data: Vec<u8>) -> SessionStats {
        SessionStats { bot, data }
    }

    pub fn parse(&self) {
        todo!("Add bot event handler");
    }
}

pub struct Accolade {}

pub struct VoiceControl {}

pub struct VoiceData {}

pub struct MinimapUpdate {}

pub struct GameStopInternal {}

pub struct BattleRoyaleAction {}

pub struct BattleRoyaleListRequest {}

pub struct BattleRoyaleListResult {}

pub struct BattleRoyaleStatusUpdate {}

pub struct BattleRoyaleResultInternal {}

pub struct AdminInternal8 {}

pub struct PingMessage {}

/// ## ?
/// Used to establish a session
/// between the client and the server.
/// Ref: Game's home screen.
pub struct ConnectRequest3<'a> {
    enum_type: enums::Packet,
    bot: &'a models::Bot<'a>,
    game_mode: enums::GameMode,
    public_first_connection: bool,
    mayhem_ticked: bool,
}

impl<'a> ConnectRequest3<'a> {
    pub fn new(
        bot: &'a models::Bot<'a>,
        game_mode: enums::GameMode,
        no_public_first_connection: bool,
        mayhem_ticked: bool,
    ) -> ConnectRequest3<'a> {
        ConnectRequest3 {
            enum_type: enums::Packet::CONNECT_REQUEST_3,
            bot,
            game_mode,
            public_first_connection: no_public_first_connection,
            mayhem_ticked,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let rng_crypt_seed = rand::thread_rng().gen::<u64>();
        let rng_crypt_key = JavaRNG::new(rng_crypt_seed).next_u64();

        let mut b_arr = net::ByteArray::new(None);

        b_arr
            .write_byte(self.enum_type as u8)
            .write_int(0x00000000)
            .write_long(rng_crypt_seed)
            .write_short(0x04A1)
            .write_int(self.bot.net.rng_token1)
            .write_byte(self.game_mode as u8)
            .write_byte(self.public_first_connection as u8)
            .write_int(0xFFFFFFFF)
            .write_utf8(self.bot.player_data.ticket)
            .write_byte(self.bot.player_data.visibility.unwrap_or_default() as u8)
            .write_bool(self.mayhem_ticked)
            .write_short(self.bot.player_data.skin.unwrap_or_default() as u16)
            .write_byte(self.bot.player_data.eject_skin.unwrap_or(0xFF) as u8)
            .write_utf8(self.bot.player_data.name)
            .write_int(0x00000000)
            .write_byte(self.bot.player_data.name.len() as u8)
            .write_raw(&vec![0xFF; self.bot.player_data.name.len()])
            .write_byte(0xFF)
            .write_int(0xFF828282)
            .write_utf8("")
            .write_byte(self.bot.player_data.hat.unwrap_or(0xFF) as u8)
            .write_int(0x00000000)
            .write_byte(self.bot.player_data.halo.unwrap_or(0x00))
            .write_byte(0xFF)
            .write_utf8("")
            .write_int(0x00000000)
            .write_int(0x00000000)
            .write_byte(self.bot.player_data.particle.unwrap_or(0xFF) as u8)
            .write_byte(self.bot.player_data.name_font.unwrap_or_default() as u8)
            .write_byte(0x05)
            .write_raw(&vec![0x05, 0x77])
            .write_byte(self.bot.player_data.rainbow_cycle.unwrap_or_default() as u8)
            .write_short(self.bot.player_data.skin.unwrap_or_default() as u16)
            .write_short(0x0000)
            .write_int(0x00000000)
            .write_long(chrono::Utc::now().timestamp_millis() as u64);

        b_arr = rng_crypt::encrypt(b_arr, rng_crypt_key);

        return b_arr.data;
    }
}

pub struct ArenaCDInternal {}