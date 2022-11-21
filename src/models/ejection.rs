#[derive(Default)]
pub struct Ejection {
    pub id: u8,
    pub player_id: u8,
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub angle: f32,
}

impl Ejection {
    pub fn new(id: u8, player_id: u8, x: f32, y: f32, mass: f32, angle: f32) -> Self {
        Ejection {
            id,
            player_id,
            x,
            y,
            mass,
            angle,
        }
    }
}
