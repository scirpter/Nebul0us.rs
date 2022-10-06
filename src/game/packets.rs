use crate::game::enums::Packet;
use crate::models;

pub struct ConnectResult2 {}

pub struct Control {}

pub struct KeepAlive {}

pub struct InvalidateClient {}

pub struct StartGameInternal {}

pub struct ConnectRequest {}

pub struct Disconnect<'a> {
    id: Packet,
    bot: models::Bot<'a>,
}

impl<'a> Disconnect<'a> {
    pub fn new(bot: models::Bot) -> Disconnect {
        Disconnect {
            id: Packet::DISCONNECT,
            bot,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut b_arr = models::ByteArray::new(None);
        b_arr.write_byte(self.id as u8);
        b_arr.write_int(self.bot.net.cr2_token1.unwrap_or_default());
        b_arr.write_int(self.bot.net.cr2_token2.unwrap_or_default());
        b_arr.write_int(self.bot.net.rng_token1);
        return b_arr.data;
    }
}

pub struct GameChatMessage {}

pub struct ClanChatMessage {}

pub struct JoinRequest {}

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

pub struct SessionStats<'a> {
    bot: models::Bot<'a>,
    data: Vec<u8>,
}

impl<'a> SessionStats<'a> {
    pub fn new(bot: models::Bot, data: Vec<u8>) -> SessionStats {
        SessionStats { bot, data }
    }

    pub fn parse(&self) {
        // TODO: Add bot event handler
        todo!()
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

pub struct ConnectRequest3 {}

pub struct ArenaCDInternal {}
