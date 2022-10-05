#[allow(non_camel_case_types)]
pub mod enums {
    mod base_event;
    mod clan_rank;
    mod click_type;
    mod color_cycle;
    mod connection_state;
    mod packet_id;

    pub use {
        base_event::BaseEvent, clan_rank::ClanRank, click_type::ClickType, color_cycle::ColorCycle,
        connection_state::ConnectionState, packet_id::PacketId,
    };
}

pub mod netrng;
