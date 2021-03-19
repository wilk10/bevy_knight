use bevy::prelude::*;

use crate::constants::Const;

pub struct Knight {
    pub size: Vec2,
    pub velocity: Vec3,
    pub initial_position: Vec3,
    pub scale: Vec3,
    pub is_in_air: bool,
}

impl Knight {
    pub fn horizontal_speed() -> f32 {
        8.0
    }

    pub fn initial_vertical_speed() -> f32 {
        16.0
    }

    pub fn scale() -> f32 {
        2.0
    }

    pub fn sprite_size() -> Vec2 {
        Vec2::new(32.0, 48.0)
    }
}

impl Default for Knight {
    fn default() -> Self {
        Knight {
            size: Knight::sprite_size() * Knight::scale(),
            velocity: Vec3::new(0.0, 0.0, 0.0),
            initial_position: Vec3::new(32.0, 70.0, 15.0),
            scale: Vec3::splat(Const::global_scale() * Knight::scale()),
            is_in_air: true,
        }
    }
}
