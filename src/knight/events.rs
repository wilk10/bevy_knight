use bevy::sprite::collide_aabb::Collision;

pub struct CollisionEvent {
    pub collision: Collision,
}
