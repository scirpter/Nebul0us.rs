#[derive(Debug, PartialEq)]
pub enum ConnectResult {
    SUCCESS,
    GAME_NOT_FOUND,
    UNKNOWN,
    ACCOUNT_ALREADY_SIGNED_IN,

    // Determine how much items there
    // are in this enum
    COUNT,
}

impl ConnectResult {
    pub fn from_u8(value: u8) -> Option<ConnectResult> {
        match value {
            0 => Some(ConnectResult::SUCCESS),
            1 => Some(ConnectResult::GAME_NOT_FOUND),
            2 => Some(ConnectResult::UNKNOWN),
            3 => Some(ConnectResult::ACCOUNT_ALREADY_SIGNED_IN),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ConnectResult::SUCCESS => 0,
            ConnectResult::GAME_NOT_FOUND => 1,
            ConnectResult::UNKNOWN => 2,
            ConnectResult::ACCOUNT_ALREADY_SIGNED_IN => 3,
            _ => 0,
        }
    }
}
