use avian2d::prelude::*;
use bevy::prelude::*;

use crate::plane::Plane;

pub const BULLET_SPEED: f32 = 700.0;
pub const BULLET_SIZE: f32 = 5.0;
pub const BULLET_LIFETIME: f32 = 2.0;

#[derive(Component)]
pub struct Bullet {
    pub lifetime: Timer,
}

pub fn system_shoot_bullets(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    planes: Query<(&Transform, &Plane)>,
) {
    for (plane_transform, plane) in planes.iter() {
        let direction = plane_transform.rotation * Vec3::X;
        let d = plane.d();

        if keyboard.just_pressed(plane.btn_shoot) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        plane_transform.translation + direction * d * 30.0,
                    ),
                    ..default()
                },
                Bullet {
                    lifetime: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
                },
                RigidBody::Dynamic,
                Collider::circle(BULLET_SIZE / 2.0),
                LinearVelocity(direction.truncate() * BULLET_SPEED * d),
            ));
        }
    }

    // if keyboard.just_pressed(KeyCode::Space) {
    //     let plane_transform = query.single();
    //     let direction = plane_transform.rotation * Vec3::X;

    //     commands.spawn((
    //         SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::WHITE,
    //                 custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
    //                 ..default()
    //             },
    //             transform: Transform::from_translation(
    //                 plane_transform.translation + direction * 30.0,
    //             ),
    //             ..default()
    //         },
    //         Bullet {
    //             lifetime: Timer::from_seconds(2.0, TimerMode::Once),
    //         },
    //         RigidBody::Dynamic,
    //         Collider::circle(BULLET_SIZE / 2.0),
    //         LinearVelocity(direction.truncate() * BULLET_SPEED),
    //     ));
    // }
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
