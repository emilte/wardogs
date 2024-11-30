use avian2d::prelude::*;
use bevy::prelude::*;

use wardogs::bullet::{system_cleanup_bullets, system_shoot_bullets};
use wardogs::ground::Ground;
use wardogs::plane::{system_simple_plane_movement, system_wrap_plane_position, Plane};
use wardogs::system_handle_collisions;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, -100.0)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                system_simple_plane_movement,
                system_shoot_bullets,
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
        Plane {
            base_speed: 100.0,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::triangle(
            Vec2::new(-plane_size.x / 2.0, -plane_size.y / 2.0),
            Vec2::new(-plane_size.x / 2.0, plane_size.y / 2.0),
            Vec2::new(plane_size.x / 2.0, 0.0),
        ),
        LinearVelocity::default(),
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
