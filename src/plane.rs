use avian2d::prelude::*;
use bevy::prelude::*;

pub const GRAVITY: f32 = -150.0;
pub const DRAG: f32 = 0.96;
pub const MAX_SPEED: f32 = 300.0;
pub const ROTATION_SPEED: f32 = 3.5;
pub const LIFT_COEFFICIENT: f32 = 0.6;
pub const ACCELERATION: f32 = 100.0;
pub const DECELERATION: f32 = 200.0;

#[derive(Component)]
pub struct Plane {
    pub base_speed: f32,
    pub current_speed: f32,
    pub vertical_velocity: f32,

    pub drag: f32,
    pub gravity: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub rotation_speed: f32,
    pub lift_coefficient: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            base_speed: 0.0,
            current_speed: 100.0,
            vertical_velocity: 0.0,

            drag: DRAG,
            gravity: GRAVITY,
            max_speed: MAX_SPEED,
            acceleration: ACCELERATION,
            deceleration: DECELERATION,
            rotation_speed: ROTATION_SPEED,
            lift_coefficient: LIFT_COEFFICIENT,
        }
    }
}

impl Plane {
    pub fn lift_v2() -> Self {
        Self {
            base_speed: 50.0,
            current_speed: 0.0,
            vertical_velocity: 0.0,

            drag: 0.98,
            gravity: GRAVITY,
            max_speed: MAX_SPEED,
            acceleration: 50.0,
            deceleration: DECELERATION,
            rotation_speed: ROTATION_SPEED,
            lift_coefficient: LIFT_COEFFICIENT,
        }
    }
}

pub fn system_plane_movement(
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
        transform.rotate_z(plane.rotation_speed * std::f32::consts::PI / 180.0);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-plane.rotation_speed * std::f32::consts::PI / 180.0);
    }

    // Handle acceleration
    if keyboard.pressed(KeyCode::ArrowUp) {
        plane.current_speed = (plane.current_speed + plane.acceleration * dt).min(plane.max_speed);
    } else {
        plane.current_speed = (plane.current_speed - plane.deceleration * dt).max(plane.base_speed);
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
    plane.vertical_velocity += plane.gravity * dt;

    plane.vertical_velocity += lift * dt;

    // Apply drag to vertical velocity
    plane.vertical_velocity *= plane.drag;

    // Combine horizontal movement and vertical velocity
    let direction = transform.rotation * Vec3::X;
    let forward_motion = direction.truncate() * plane.current_speed;
    velocity.0 = Vec2::new(forward_motion.x, forward_motion.y + plane.vertical_velocity);
}

pub fn system_simple_plane_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut Plane)>,
) {
    let (mut transform, mut velocity, mut plane) = query.single_mut();

    // Rotate up/down
    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.rotate_z(plane.rotation_speed * std::f32::consts::PI / 180.0);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.rotate_z(-plane.rotation_speed * std::f32::consts::PI / 180.0);
    }

    // Handle acceleration
    if keyboard.pressed(KeyCode::ArrowUp) {
        plane.current_speed =
            (plane.current_speed + plane.acceleration * time.delta_seconds()).min(plane.max_speed);
    } else {
        plane.current_speed =
            (plane.current_speed - plane.acceleration * time.delta_seconds()).max(plane.base_speed);
    }

    // Apply thrust in the direction the plane is facing
    let direction = transform.rotation * Vec3::X;
    velocity.0 = direction.truncate() * plane.current_speed;
}

pub fn system_wrap_plane_position(
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