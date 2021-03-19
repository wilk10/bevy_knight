use bevy::prelude::*;

use crate::{
    knight::{
        events::CollisionEvent,
        systems::{
            collision::detect_collision,
            input::handle_movement,
            spawn::spawn_knight,
        },
    },
    load::plugin::LoadLabel,
};

pub struct JustMoved;

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CollisionEvent>()
            .add_startup_system(spawn_knight.system().after(LoadLabel))
            .add_system(handle_movement.system())
            .add_system(detect_collision.system());
    }
}
