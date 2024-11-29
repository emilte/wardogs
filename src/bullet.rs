use avian2d::prelude::*;
use bevy::prelude::*;

use crate::plane::Plane;

#[derive(Component)]
pub struct Bullet {
    pub lifetime: Timer,
}

pub fn system_shoot_bullets(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Plane>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let plane_transform = query.single();
        let direction = plane_transform.rotation * Vec3::X;
        const BULLET_SPEED: f32 = 500.0;
        const BULLET_SIZE: f32 = 5.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(
                    plane_transform.translation + direction * 30.0,
                ),
                ..default()
            },
            Bullet {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
            RigidBody::Dynamic,
            Collider::circle(BULLET_SIZE / 2.0),
            LinearVelocity(direction.truncate() * BULLET_SPEED),
        ));
    }
}

pub fn system_cleanup_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
