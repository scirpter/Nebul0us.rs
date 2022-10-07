pub enum PlayerVisibility {
    ONLINE,
    APPEAR_OFFLINE,
    HIDDEN,
    DND,
}

impl Default for PlayerVisibility {
    fn default() -> Self {
        PlayerVisibility::ONLINE
    }
}

impl Copy for PlayerVisibility {}

impl Clone for PlayerVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
