#[allow(non_camel_case_types)]
pub mod enums {
    mod base_event;
    pub use base_event::BaseEvent;

    mod clan_rank;
    pub use clan_rank::ClanRank;

    mod click_type;
    pub use click_type::ClickType;

    mod color_cycle;
    pub use color_cycle::ColorCycle;

    mod connection_state;
    pub use connection_state::ConnectionState;
}

pub mod netrng;
pub mod packets;
