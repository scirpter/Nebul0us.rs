pub enum ColorCycle {
    NONE,
    COLOR_CYCLE_SLOW,
    COLOR_CYCLE_FAST,
    RAINBOW_HORIZONTAL_SLOW,
    RAINBOW_HORIZONTAL_FAST,
    RAINBOW_VERTICAL_SLOW,
    RAINBOW_VERTICAL_FAST,
}

impl Default for ColorCycle {
    fn default() -> Self {
        ColorCycle::NONE
    }
}

impl Copy for ColorCycle {}

impl Clone for ColorCycle {
    fn clone(&self) -> Self {
        *self
    }
}
