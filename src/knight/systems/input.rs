use bevy::{
    prelude::*,
    sprite::collide_aabb::Collision,
};

use crate::{
    constants::Const,
    knight::{
        components::knight::Knight,
        events::CollisionEvent,
    },
};

pub fn handle_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut knight_query: Query<(&mut Knight, &mut Transform)>,
    mut events: EventReader<CollisionEvent>,
) {
    for (mut knight, mut knight_transform) in knight_query.iter_mut() {
        if keys.pressed(KeyCode::Right) {
            knight.velocity.x = Knight::horizontal_speed();
            knight_transform.rotation = Quat::from_rotation_y(0.);
        }

        if keys.pressed(KeyCode::Left) {
            knight.velocity.x = -Knight::horizontal_speed();
            knight_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }

        if keys.just_released(KeyCode::Right) || keys.just_released(KeyCode::Left) {
            knight.velocity.x = 0.0;
        }

        if keys.pressed(KeyCode::Up) {
            if !knight.is_in_air {
                knight.velocity.y = Knight::initial_vertical_speed();
                knight.is_in_air = true;
            }
        }

        if knight.is_in_air {
            knight.velocity -= Vec3::new(0., Const::gravity() * time.delta_seconds(), 0.);
        }

        if knight.velocity != Vec3::ZERO {
            for collision_event in events.iter() {
                match collision_event.collision {
                    Collision::Top => knight.velocity *= Vec3::new(1., 0., 1.),
                    Collision::Bottom => {
                        knight.velocity *= Vec3::new(1., 0., 1.);
                        knight.is_in_air = false;
                    }
                    Collision::Left => knight.velocity *= Vec3::new(0., 1., 1.),
                    Collision::Right => knight.velocity *= Vec3::new(0., 1., 1.),
                }
            }
        }

        let next_translation =
            knight_transform.translation + knight.velocity * Const::speed() * time.delta_seconds();
        knight_transform.translation = next_translation;
    }
}
