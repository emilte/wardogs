// use bevy::prelude::*;

// use crate::{
//     physics::Velocity,
//     player::{FaceInDirection, Lifetime, Player},
// };

// #[derive(Component)]
// struct Bullet;

// pub fn player_shooting_system(
//     mut commands: Commands,
//     mut players: Query<(&Transform, &Velocity, &mut Player)>,
//     keys: Res<Input<KeyCode>>,
//     assets: Res<AssetServer>,
//     time: Res<Time>,
// ) {
//     let bullet_speed = 500.0;
//     for (transform, velocity, mut player) in players.iter_mut() {
//         if !keys.pressed(player.button_shoot) {
//             continue;
//         }
//         player.shooting_timer -= time.delta_seconds();
//         if player.shooting_timer > 0.0 {
//             continue;
//         }
//         player.shooting_timer = 0.5;

//         let dir = velocity.0.normalize() * bullet_speed;

//         commands.spawn((
//             Bullet,
//             Lifetime(3.0),
//             Velocity(dir),
//             FaceInDirection,
//             SpriteBundle {
//                 texture: assets.load("../assets/laser.png"),
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::new(10.0, 64.0)),
//                     ..default()
//                 },
//                 transform: Transform::default().with_translation(transform.translation),
//                 ..default()
//             },
//         ));
//     }
// }
