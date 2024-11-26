use bevy::prelude::*;
use dotenv::dotenv;
use std::env;

mod bullet;
mod physics;
mod player;

fn main() {
    dotenv().ok();
    let app_mode = env::var("APP_MODE").unwrap_or_else(|_| "default".to_string());

    if app_mode == "emil" {
        emil();
    } else {
        app();
    }
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    println!("Started Game.");
}

fn app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Startup, player::spawn_player_system)
        .add_systems(PreUpdate, bullet::player_shooting_system)
        .add_systems(
            Update,
            (
                physics::friction_system,
                physics::velocity_system,
                player::control_player_system,
                player::crazy_player_particle_system,
                player::face_in_direction_system,
                player::lifetime_system,
            ),
        )
        .run();
}

fn emil() {
    println!("Emil mode");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Startup, player::spawn_player_system)
        .add_systems(PreUpdate, bullet::player_shooting_system)
        .add_systems(
            Update,
            (
                physics::friction_system,
                physics::velocity_system,
                player::control_player_system,
                // player::crazy_player_particle_system,
                player::face_in_direction_system,
                player::lifetime_system,
            ),
        )
        .run();
}
