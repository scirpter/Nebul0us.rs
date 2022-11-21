use std::collections::HashMap;

use crate::models;

use super::Ejection;

#[derive(Default)]
pub struct World {
    pub tick: u8,
    pub raw_size: f32,
    // hashmap
    pub ejections: HashMap<u8, Ejection>,
}

impl World {
    pub fn default() -> Self {
        World {
            ..Default::default()
        }
    }
}
