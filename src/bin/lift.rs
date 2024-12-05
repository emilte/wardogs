use avian2d::prelude::*;
use bevy::prelude::*;

use wardogs::{
    bullet::{system_cleanup_bullets, system_handle_bullet_hits, system_shoot_bullets},
    ground::{system_handle_collisions, Ground},
    plane::{system_plane_movement, system_wrap_plane_position, Plane, PlaneDirection},
};

const GRAVITY: f32 = -100.0;

#[derive(PhysicsLayer)]
enum GameLayer {
    Player, // Layer 0
    Enemy,  // Layer 1
    Ground, // Layer 2
}

// Player collides with enemies and the ground, but not with other players

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, GRAVITY)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                system_plane_movement,
                system_shoot_bullets,
                system_handle_bullet_hits,
                system_cleanup_bullets,
                system_handle_collisions,
                system_wrap_plane_position,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, window_query: Query<&Window>, asset_server: Res<AssetServer>) {
    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    let plane_texture = asset_server.load("plane.png");

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Load the plane texture
    let plane_size = Vec2::new(40.0, 20.0);

    let play_layers =
        CollisionLayers::new(GameLayer::Player, [GameLayer::Enemy, GameLayer::Ground]);

    // Plane with sprite
    commands.spawn((
        SpriteBundle {
            texture: plane_texture.clone(),
            sprite: Sprite {
                custom_size: Some(plane_size),
                flip_x: false,
                // flip_y: true,
                ..default()
            },
            transform: Transform::from_xyz(300.0, 200.0, 0.0),
            ..default()
        },
        Plane {
            dir: PlaneDirection::LEFT,
            ..Plane::lift_v2()
        },
        RigidBody::Dynamic,
        Collider::rectangle(plane_size.x / 2.0, plane_size.y / 2.0),
        LinearVelocity::default(),
        CollidingEntities::default(),
        play_layers,
    ));

    // Opponent Plane with sprite.
    commands.spawn((
        SpriteBundle {
            texture: plane_texture,
            sprite: Sprite {
                custom_size: Some(plane_size),
                flip_x: true,
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 200.0, 0.0),
            ..default()
        },
        Plane {
            btn_left: KeyCode::KeyA,
            btn_right: KeyCode::KeyD,
            btn_boost: KeyCode::KeyW,
            btn_shoot: KeyCode::ShiftLeft,
            ..Plane::lift_v2()
        },
        RigidBody::Dynamic,
        Collider::rectangle(plane_size.x / 2.0, plane_size.y / 2.0),
        LinearVelocity::default(),
        CollidingEntities::default(),
        play_layers,
    ));

    // Ground
    const GROUND_HEIGHT: f32 = 40.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0., 10., 0.),
                custom_size: Some(Vec2::new(window_width + 100.0, GROUND_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -window_height / 2.0, 0.0),
            ..default()
        },
        Ground,
        RigidBody::Static,
        Collider::rectangle(window_width + 100.0, GROUND_HEIGHT),
        CollidingEntities::default(),
    ));
}
