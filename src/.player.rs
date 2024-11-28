use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use rand::Rng;

use crate::{
    physics::{Friction, Velocity},
    MultiplayerEvent, MyClientId, ServerPlayer,
};

#[derive(Component)]
pub struct Player {
    id: u64,
    move_speed: f32,
    rotation_speed: f32,
    button_left: KeyCode,
    button_right: KeyCode,
    pub button_shoot: KeyCode,
    particle_timer: f32,
    pub shooting_timer: f32,
}

#[derive(Component)]
pub struct ControllablePlayer {
    update_timer: Timer,
}

#[derive(Component)]
pub struct FaceInDirection;

#[derive(Component)]
pub struct Lifetime(pub f32);

pub fn spawn_player_system(
    mut events: EventReader<MultiplayerEvent>,
    mut cmd: Commands,
    my_client_id: Res<MyClientId>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        let MultiplayerEvent::PlayerCreated(player) = event else {
            continue;
        };
        let mut player_ent = cmd.spawn((
            Player {
                id: player.client_id,
                move_speed: 15.0,
                rotation_speed: 160f32.to_radians(), // Degrees/second
                button_left: KeyCode::ArrowLeft,
                button_right: KeyCode::ArrowRight,
                button_shoot: KeyCode::ArrowUp,
                particle_timer: 0.0,
                shooting_timer: 0.0,
            },
            Velocity::default(),
            FaceInDirection,
            SpriteBundle {
                texture: assets.load("../assets/ship.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_xyz(player.position.x, player.position.y, 0.0),
                ..default()
            },
        ));
        // If self player, add controllable
        if player.client_id == my_client_id.0 {
            let update_frequency = 1.0 / 10.0;
            player_ent.insert(ControllablePlayer {
                update_timer: Timer::from_seconds(update_frequency, TimerMode::Repeating),
            });
        }
    }
}

pub fn control_player_system(
    mut query: Query<(&Player, &mut Velocity), With<ControllablePlayer>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (player, mut velocity) in query.iter_mut() {
        let current_dir = velocity.0.to_angle();
        let rotate_dir = match (
            keys.pressed(player.button_left),
            keys.pressed(player.button_right),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let new_dir = current_dir + rotate_dir * player.rotation_speed * dt;
        velocity.0 = Vec2::new(new_dir.cos(), new_dir.sin()) * player.move_speed;
    }
}

// Update player position for server
pub fn sync_player_with_server_system(
    mut query: Query<(&Player, &mut Transform, &mut ControllablePlayer)>,
    mut client: ResMut<RenetClient>,
    time: Res<Time>,
) {
    for (player, transform, mut controller) in query.iter_mut() {
        controller.update_timer.tick(time.delta());
        if !controller.update_timer.finished() {
            continue;
        }
        let player_move = MultiplayerEvent::PlayerMoved(ServerPlayer {
            client_id: player.id,
            position: transform.translation.xy(),
        });
        client.send_message(DefaultChannel::Unreliable, player_move.bytes());
    }
}

pub fn sync_other_player_positions_system(
    mut query: Query<(&Player, &mut Transform), Without<ControllablePlayer>>,
    mut events: EventReader<MultiplayerEvent>,
) {
    for evt in events.read() {
        let MultiplayerEvent::PlayerMoved(moved) = evt else {
            continue;
        };
        for (player, mut transform) in query.iter_mut() {
            if player.id != moved.client_id {
                continue;
            }
            transform.translation =
                Vec3::new(moved.position.x, moved.position.y, transform.translation.z);
        }
    }
}

pub fn face_in_direction_system(
    mut query: Query<(&Velocity, &mut Transform), With<FaceInDirection>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        *transform = transform.with_rotation(Quat::from_axis_angle(
            Vec3::Z,
            velocity.0.to_angle() + 90f32.to_radians(),
        ));
    }
}

/// Deletes an entity after lifetime runs out.
pub fn lifetime_system(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
        if lifetime.0 <= 0.0 {
            cmd.entity(entity).despawn();
        }
    }
}

pub fn crazy_player_particle_system(
    mut cmd: Commands,
    mut query: Query<(&mut Player, &Transform)>,
    time: Res<Time>,
    assets: Res<AssetServer>,
) {
    let spawn_delay = 1.0 / 30.0;
    let mut rnd = rand::thread_rng();
    for (mut player, transform) in query.iter_mut() {
        player.particle_timer += time.delta_seconds();
        while player.particle_timer >= spawn_delay {
            player.particle_timer -= spawn_delay;

            let random_angle = rnd.gen_range(0f32..360f32).to_radians();
            let random_speed = rnd.gen_range(50.0..150.0);
            cmd.spawn((
                Lifetime(rnd.gen_range(1.5..3.5)),
                Velocity(Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed),
                Friction(500.0),
                SpriteBundle {
                    texture: assets.load("../assets/bubble.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(12.0)),
                        color: Color::BLACK,
                        ..default()
                    },
                    transform: transform.with_translation(transform.translation.with_z(-1.0)),
                    ..default()
                },
            ));
        }
    }
}
