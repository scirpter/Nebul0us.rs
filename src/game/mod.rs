#[allow(non_camel_case_types)]
pub mod enums {
    mod base_event;
    mod clan_rank;
    mod click_type;
    mod color_cycle;
    mod connect_result;
    mod connection_state;
    mod difficulty;
    mod game_mode;
    mod name_font;
    mod packet;
    mod player_visibility;
    mod server;
    mod skin;
    mod split_multiplier;
    mod world_size;

    pub use {
        base_event::BaseEvent, clan_rank::ClanRank, click_type::ClickType, color_cycle::ColorCycle,
        connect_result::ConnectResult, connection_state::ConnectionState, difficulty::Difficulty,
        game_mode::GameMode, name_font::NameFont, packet::*, player_visibility::PlayerVisibility,
        server::Server, skin::Skin, split_multiplier::SplitMultiplier, world_size::WorldSize,
    };
}

pub mod packets {
    mod clan_chat_message;
    mod client_preferences;
    mod connect_request;
    mod connect_request_3;
    mod connect_result_2;
    mod control;
    mod disconnect;
    mod emote_request;
    mod enter_game_request;
    mod enter_game_result;
    mod game_chat_message;
    mod game_data;
    mod game_update;
    mod join_request;
    mod join_result;
    mod keep_alive;
    mod session_stats;
    mod spectate_game_request;
    mod top_scores;

    pub use {
        clan_chat_message::ClanChatMessage, client_preferences::ClientPreferences,
        connect_request::ConnectRequest, connect_request_3::ConnectRequest3,
        connect_result_2::ConnectResult2, control::Control, disconnect::Disconnect,
        emote_request::EmoteRequest, enter_game_request::EnterGameRequest,
        enter_game_result::EnterGameResult, game_chat_message::GameChatMessage,
        game_data::GameData, game_update::GameUpdate, join_request::JoinRequest,
        join_result::JoinResult, keep_alive::KeepAlive, session_stats::SessionStats,
        spectate_game_request::SpectateGameRequest, top_scores::TopScores,
    };
}
mod funx;
pub use {funx::CONNECT_REQUEST_3, funx::KEEP_ALIVE};
pub mod rng_crypt;
