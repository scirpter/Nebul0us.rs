#[derive(Default, Copy, Clone)]
pub enum PlayerVisibility {
    #[default]
    ONLINE,
    APPEAR_OFFLINE,
    HIDDEN,
    DND,
}
