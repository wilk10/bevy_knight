use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};

use crate::{
    knight::{
        components::knight::Knight,
        events::CollisionEvent,
    },
    level::components::tile::Tile,
};

pub fn detect_collision(
    mut events: EventWriter<CollisionEvent>,
    knight_query: Query<(&Knight, &Transform)>,
    tile_query: Query<(&Tile, &Transform)>,
) {
    if let Some((knight, knight_transform)) = knight_query.iter().next() {
        for (tile, tile_transform) in tile_query.iter() {
            let tile_translation = tile_transform.translation;
            let collision = collide(
                tile_translation,
                tile.size,
                knight_transform.translation,
                knight.size,
            );

            if let Some(collision) = collision {
                events.send(CollisionEvent { collision });
            }
        }
    }
}
