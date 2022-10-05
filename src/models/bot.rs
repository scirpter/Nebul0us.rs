use crate::game;

struct Control {
    // Values are cached to recover the original values,
    // if necessary
    speed: u8,
    cached_speed: u8,
    angle: u16,
    cached_angle: u16,
    eject_count: u32,
    split_count: u32,
    drop_count: u32,
}

struct PlayerData<'a> {
    name: &'a str,
    ticket: &'a str,
    skin_index: u16,
}

struct Net {
    connection_state: game::enums::ConnectionState,

    // Two tokens received from CONNECT_RESULT_2 (0x01).
    // Used to identify the client server-side
    cr2_token1: u32,
    cr2_token2: u32,

    // Same as above, but randomly generated by ourselves.
    // These also need to be provided sometimes when
    // sending packets, e.g. DISCONNECT (0x07)
    rnd_token1: u32,
    rnd_token2: u32,
}

pub struct Bot<'a> {
    control: Control,
    player_data: PlayerData<'a>,
}
