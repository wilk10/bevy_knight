use bevy::prelude::*;

pub struct Const;

impl Const {
    pub fn global_scale() -> f32 {
        1.0
    }

    pub fn gravity() -> f32 {
        32.0
    }

    pub fn map_n_tiles() -> Vec2 {
        Vec2::new(80.0, 23.0)
    }

    pub fn tile_size() -> Vec2 {
        Vec2::new(32.0, 32.0)
    }

    pub fn speed() -> f32 {
        20.0
    }
}
