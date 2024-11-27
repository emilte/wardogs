// use bevy::prelude::*;
// use bevy_renet::{
//     renet::{
//         transport::{ClientAuthentication, NetcodeClientTransport},
//         ConnectionConfig, DefaultChannel, RenetClient,
//     },
//     transport::NetcodeClientPlugin,
//     RenetClientPlugin,
// };
// use std::{net::UdpSocket, time::SystemTime};
// use wardogs::{bullet, physics, player, MultiplayerEvent, MyClientId, PROTOCOL_ID};

// fn main() {
//     let (client_id, client, transport) = new_renet_client();
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(RenetClientPlugin)
//         .add_plugins(NetcodeClientPlugin)
//         .add_systems(PreUpdate, bullet::player_shooting_system)
//         .add_systems(Startup, setup_game)
//         .add_systems(
//             Update,
//             (
//                 physics::friction_system,
//                 physics::velocity_system,
//                 player::control_player_system,
//                 // player::crazy_player_particle_system,
//                 player::face_in_direction_system,
//                 player::lifetime_system,
//                 player::spawn_player_system,
//                 player::sync_player_with_server_system,
//                 player::sync_other_player_positions_system,
//                 receive_message_system,
//                 receive_reliable_message_system,
//             ),
//         )
//         .insert_resource(client)
//         .insert_resource(transport)
//         .insert_resource(MyClientId(client_id))
//         .add_event::<MultiplayerEvent>()
//         .run();
// }

// fn new_renet_client() -> (u64, RenetClient, NetcodeClientTransport) {
//     let server_addr = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
//     let client_id = rand::random();

//     let authentication = ClientAuthentication::Unsecure {
//         client_id,
//         protocol_id: PROTOCOL_ID,
//         server_addr,
//         user_data: None,
//     };

//     let current_time = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap();

//     let client = RenetClient::new(ConnectionConfig::default());
//     let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

//     (client_id, client, transport)
// }

// fn setup_game(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
//     println!("Started Game.");
// }

// fn receive_message_system(
//     mut client: ResMut<RenetClient>,
//     mut events: EventWriter<MultiplayerEvent>,
// ) {
//     while let Some(msg) = client.receive_message(DefaultChannel::Unreliable) {
//         let message = bincode::deserialize(&msg).expect("Failed to deserialize message");
//         events.send(message);
//     }
// }

// fn receive_reliable_message_system(
//     mut client: ResMut<RenetClient>,
//     mut events: EventWriter<MultiplayerEvent>,
// ) {
//     while let Some(msg) = client.receive_message(DefaultChannel::ReliableUnordered) {
//         let message = bincode::deserialize(&msg).expect("Failed to deserialize message");
//         events.send(message);
//     }
// }
