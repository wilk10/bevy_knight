use bevy::prelude::*;

use crate::knight::events::CollisionEvent;
use crate::knight::systems::collision::detect_collision;
use crate::knight::systems::input::handle_movement;
use crate::knight::systems::spawn::spawn_knight;
use crate::load::plugin::LoadLabel;

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
