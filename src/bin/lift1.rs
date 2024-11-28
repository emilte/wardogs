use avian2d::prelude::*;
use bevy::prelude::*;

const GRAVITY: f32 = -100.0;
const PLANE_GRAVITY: f32 = -150.0;
const DRAG: f32 = 0.96;
const ROTATION_SPEED: f32 = 3.5;
const LIFT_COEFFICIENT: f32 = 0.6;
const ACCELERATION: f32 = 100.0;
const FRICTION: f32 = ACCELERATION * 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, GRAVITY)))
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
struct Plane {
    base_speed: f32,
    current_speed: f32,
    max_speed: f32,
    acceleration: f32,
    friction: f32,
    lift_coefficient: f32,
    vertical_velocity: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            base_speed: 0.0,
            current_speed: 100.0,
            max_speed: 300.0,
            acceleration: ACCELERATION,
            friction: FRICTION,
            lift_coefficient: LIFT_COEFFICIENT,
            vertical_velocity: 0.0,
        }
    }
}

#[derive(Component)]
struct Bullet {
    lifetime: Timer,
}

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Ground;

fn setup(mut commands: Commands, window_query: Query<&Window>, asset_server: Res<AssetServer>) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Load the plane texture
    let plane_texture = asset_server.load("plane.png");
    let plane_size = Vec2::new(40.0, 20.0);

    // Plane with sprite
    commands.spawn((
        SpriteBundle {
            texture: plane_texture,
            sprite: Sprite {
                custom_size: Some(plane_size),
                flip_x: true,
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 0.0, 0.0),
            ..default()
        },
        Plane::default(),
        RigidBody::Dynamic,
        Collider::triangle(
            Vec2::new(-plane_size.x / 2.0, -plane_size.y / 2.0),
            Vec2::new(-plane_size.x / 2.0, plane_size.y / 2.0),
            Vec2::new(plane_size.x / 2.0, 0.0),
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
        Ground,
        RigidBody::Static,
        Collider::rectangle(window_width + 100.0, 20.0),
        CollidingEntities::default(),
    ));
}

fn plane_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut Plane)>,
) {
    let (mut transform, mut velocity, mut plane) = query.single_mut();
    let dt = time.delta_seconds();

    // Get current angle
    let angle = transform.rotation.to_euler(EulerRot::XYZ).2;

    // Rotate up/down
    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.rotate_z(ROTATION_SPEED * std::f32::consts::PI / 180.0);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-ROTATION_SPEED * std::f32::consts::PI / 180.0);
    }

    // Handle acceleration
    if keyboard.pressed(KeyCode::ArrowUp) {
        plane.current_speed = (plane.current_speed + plane.acceleration * dt).min(plane.max_speed);
    } else {
        plane.current_speed = (plane.current_speed - plane.friction * dt).max(plane.base_speed);
    }

    // Calculate lift based on speed and angle
    // let lift = f32::max(
    //     0.0,
    //     plane.current_speed * plane.lift_coefficient * angle.cos(),
    // );
    // let lift = (plane.current_speed * plane.lift_coefficient * angle.cos()).abs();
    let mut lift = plane.current_speed * plane.lift_coefficient * angle.cos();
    if lift < 0.0 {
        lift /= -2.0;
    }
    // info!(lift);

    // Update vertical velocity with gravity and lift
    plane.vertical_velocity += PLANE_GRAVITY * dt;

    plane.vertical_velocity += lift * dt;

    // Apply drag to vertical velocity
    plane.vertical_velocity *= DRAG;

    // Combine horizontal movement and vertical velocity
    let direction = transform.rotation * Vec3::X;
    let forward_motion = direction.truncate() * plane.current_speed;
    velocity.0 = Vec2::new(forward_motion.x, forward_motion.y + plane.vertical_velocity);
}

fn shoot_bullets(
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
    ground_query: Query<Entity, With<Ground>>,
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

        // Wrap vertically ( will crash in the ground).
        if transform.translation.y > half_height * 1.1 {
            transform.translation.y = -half_height;
        }
    }
}