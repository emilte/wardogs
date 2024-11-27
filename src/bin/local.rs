use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, -100.0)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                plane_movement,
                shoot_bullets,
                cleanup_bullets,
                handle_collisions,
                wrap_plane_position,
            ),
        )
        .run();
}

#[derive(Component)]
struct Plane;

#[derive(Component)]
struct Bullet {
    lifetime: Timer,
}

#[derive(Component)]
struct Target;

fn setup(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Plane (triangle)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(40.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..default()
        },
        Plane,
        RigidBody::Dynamic,
        Collider::triangle(
            Vec2::new(-20.0, -10.0),
            Vec2::new(-20.0, 10.0),
            Vec2::new(20.0, 0.0),
        ),
        LinearVelocity::default(),
        CollidingEntities::default(),
    ));

    // Target
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(300.0, 200.0, 0.0),
            ..default()
        },
        Target,
        RigidBody::Static,
        Collider::rectangle(30.0, 30.0),
        CollidingEntities::default(),
    ));

    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(window_width + 100.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -window_height / 2.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(window_width + 100.0, 20.0),
        CollidingEntities::default(),
    ));
}

fn plane_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut LinearVelocity), With<Plane>>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    const ROTATION_SPEED: f32 = 2.0;
    const THRUST: f32 = 200.0;

    // Rotate up/down
    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.rotate_z(ROTATION_SPEED * std::f32::consts::PI / 180.0);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-ROTATION_SPEED * std::f32::consts::PI / 180.0);
    }

    // Apply thrust in the direction the plane is facing
    let direction = transform.rotation * Vec3::X;
    velocity.0 = direction.truncate() * THRUST;
}

fn shoot_bullets(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Plane>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let plane_transform = query.single();
        let direction = plane_transform.rotation * Vec3::X;
        const BULLET_SPEED: f32 = 400.0;
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

fn cleanup_bullets(
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

fn handle_collisions(
    mut commands: Commands,
    planes: Query<(Entity, &CollidingEntities), With<Plane>>,
    bullets: Query<(Entity, &CollidingEntities), With<Bullet>>,
    targets: Query<(Entity, &Transform), With<Target>>,
    ground_query: Query<
        Entity,
        (
            With<RigidBody>,
            Without<Plane>,
            Without<Target>,
            Without<Bullet>,
        ),
    >,
) {
    let ground_entity = ground_query.single();

    // Check plane collisions with ground only
    for (plane_entity, colliding) in &planes {
        if colliding.contains(&ground_entity) {
            commands.entity(plane_entity).despawn();
        }
    }

    // Check bullet collisions with target
    if let Ok((target_entity, _)) = targets.get_single() {
        for (bullet_entity, colliding) in &bullets {
            if colliding.contains(&target_entity) {
                commands.entity(bullet_entity).despawn();
                commands.entity(target_entity).despawn();
            }
        }
    }
}

fn wrap_plane_position(
    mut query: Query<&mut Transform, With<Plane>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for mut transform in &mut query {
        // Wrap horizontally
        if transform.translation.x > half_width {
            transform.translation.x = -half_width;
        } else if transform.translation.x < -half_width {
            transform.translation.x = half_width;
        }

        // Wrap vertically
        if transform.translation.y > half_height {
            transform.translation.y = -half_height;
        } else if transform.translation.y < -half_height {
            transform.translation.y = half_height;
        }
    }
}