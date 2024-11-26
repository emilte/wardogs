use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Friction(pub f32);

pub fn friction_system(mut query: Query<(&mut Velocity, &Friction)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut velocity, friction) in query.iter_mut() {
        let velocity = &mut velocity.0;
        if velocity.x > 0.0 {
            velocity.x = f32::max(0.0, velocity.x - friction.0 * dt);
        } else if velocity.x < 0.0 {
            velocity.x = f32::min(velocity.x + friction.0 * dt, 0.0);
        }
        if velocity.y > 0.0 {
            velocity.y = f32::max(0.0, velocity.y - friction.0 * dt);
        } else if velocity.y < 0.0 {
            velocity.y = f32::min(velocity.y + friction.0 * dt, 0.0);
        }
    }
}

pub fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        let mut new_pos = transform.translation;
        new_pos.x += velocity.0.x * dt;
        new_pos.y += velocity.0.y * dt;
        *transform = transform.with_translation(new_pos);
    }
}
