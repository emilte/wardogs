use avian2d::prelude::*;
use bevy::prelude::*;
use bullet::Bullet;
use ground::Ground;
use plane::Plane;

// Define or import the Target type

pub mod bullet;
pub mod ground;
pub mod plane;

pub fn system_handle_collisions(
    mut commands: Commands,
    planes: Query<(Entity, &CollidingEntities), With<Plane>>,
    bullets: Query<(Entity, &CollidingEntities), With<Bullet>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let ground_entity = ground_query.single();

    // Check plane collisions with ground only
    for (plane_entity, colliding) in &planes {
        if colliding.contains(&ground_entity) {
            commands.entity(plane_entity).despawn();
        }
        for (bullet_entity, colliding) in &bullets {
            if colliding.contains(&plane_entity) {
                commands.entity(bullet_entity).despawn();
                commands.entity(plane_entity).despawn();
            }
        }
    }
}
