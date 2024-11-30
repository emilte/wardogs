use avian2d::prelude::*;
use bevy::prelude::*;

use crate::plane::Plane;

#[derive(Component)]
pub struct Ground;

pub fn system_handle_collisions(
    mut commands: Commands,
    planes: Query<(Entity, &CollidingEntities), With<Plane>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    let ground_entity = ground_query.single();

    // Check plane collisions with ground only
    for (plane_entity, colliding) in &planes {
        if colliding.contains(&ground_entity) {
            commands.entity(plane_entity).despawn();
        }
    }
}
