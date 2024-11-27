// use bevy::prelude::*;
// use bevy_renet::renet::Bytes;
// use serde::*;

// pub mod bullet;
// pub mod physics;
// pub mod player;

// /// Protocol ID needs to be the same across client/server.
// pub const PROTOCOL_ID: u64 = 0;

// #[derive(Resource)]
// pub struct MyClientId(pub u64);

// #[derive(Debug, Serialize, Deserialize, Event)]
// pub enum MultiplayerEvent {
//     Ping { hello: String },
//     PlayerCreated(ServerPlayer),
//     PlayerMoved(ServerPlayer),
//     PlayerDeleted(u64),
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ServerPlayer {
//     pub client_id: u64,
//     pub position: Vec2,
// }

// impl MultiplayerEvent {
//     pub fn bytes(self) -> Bytes {
//         bincode::serialize(&self)
//             .expect("Failed to serialize multiplayer event")
//             .into()
//     }
// }
