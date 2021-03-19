use bevy::prelude::*;

use crate::constants::Const;

pub struct Tile {
    pub size: Vec2,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            size: Const::tile_size() * Const::global_scale(),
        }
    }
}
