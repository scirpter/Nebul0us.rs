use crate::game::enums;

pub struct WorldProps<'a> {
    is_hidden: bool,
    min_players: u8,
    max_players: u8,
    game_mode: enums::GameMode,
    world_size: enums::WorldSize,
    difficulty: enums::Difficulty,
    room_name: &'a str,
    duration: u16,
    mayhem: bool,
    split_multiplier: enums::SplitMultiplier,
    allow_ultra_click: bool,
    allow_mass_boost: bool,
    rainbow_holes: bool,
    walls: u8,
    admin: bool,
    guests: bool,
    arena: bool,
}
