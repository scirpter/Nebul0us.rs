#[derive(Default, Copy, Clone)]
pub enum ColorCycle {
    #[default]
    NONE,
    COLOR_CYCLE_SLOW,
    COLOR_CYCLE_FAST,
    RAINBOW_HORIZONTAL_SLOW,
    RAINBOW_HORIZONTAL_FAST,
    RAINBOW_VERTICAL_SLOW,
    RAINBOW_VERTICAL_FAST,
}
