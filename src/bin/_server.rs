// use bevy::app::*;
// use bevy::prelude::*;
// use bevy::utils::HashMap;
// use bevy_renet::renet::transport::NetcodeServerTransport;
// use bevy_renet::renet::transport::ServerAuthentication;
// use bevy_renet::renet::transport::ServerConfig;
// use bevy_renet::renet::ClientId;
// use bevy_renet::renet::ConnectionConfig;
// use bevy_renet::renet::DefaultChannel;
// use bevy_renet::renet::RenetServer;
// use bevy_renet::renet::ServerEvent;
// use bevy_renet::transport::NetcodeServerPlugin;
// use bevy_renet::RenetServerPlugin;
// use std::net::UdpSocket;
// use std::time::SystemTime;
// use wardogs::MultiplayerEvent;
// use wardogs::ServerPlayer;
// use wardogs::PROTOCOL_ID;

// #[derive(Resource, Default)]
// pub struct ServerLobby {
//     pub players: HashMap<u64, ServerPlayer>,
// }

// fn main() {
//     let (server, transport) = new_renet_server();
//     App::new()
//         .add_plugins(MinimalPlugins)
//         .add_plugins(RenetServerPlugin)
//         .add_plugins(NetcodeServerPlugin)
//         .insert_resource(server)
//         .insert_resource(transport)
//         .insert_resource(ServerLobby::default())
//         .add_event::<MultiplayerEvent>()
//         .add_systems(Update, handle_events_system)
//         .add_systems(Update, (receive_message_system, sync_player_positions))
//         // .add_systems(Update, send_message_system)
//         // .add_systems(Update, server_ping)
//         .run();
// }

// fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
//     let server_addr = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind(server_addr).unwrap();
//     let current_time = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap();

//     let server_config = ServerConfig {
//         current_time,
//         max_clients: 4,
//         protocol_id: PROTOCOL_ID,
//         authentication: ServerAuthentication::Unsecure,
//         public_addresses: vec![server_addr],
//     };

//     let server = RenetServer::new(ConnectionConfig::default());
//     // let server = RenetServer::new(server_config);
//     let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

//     (server, transport)
// }

// fn handle_events_system(
//     mut server: ResMut<RenetServer>,
//     mut server_events: EventReader<ServerEvent>,
//     mut lobby: ResMut<ServerLobby>,
// ) {
//     for event in server_events.read() {
//         match event {
//             ServerEvent::ClientConnected { client_id } => {
//                 println!("Client {client_id} connected");
//                 let new_player = ServerPlayer {
//                     client_id: client_id.raw(),
//                     position: Vec2::new(-500.0, 0.0),
//                 };
//                 // Broadcast new player to others
//                 server.broadcast_message(
//                     DefaultChannel::ReliableUnordered,
//                     MultiplayerEvent::PlayerCreated(new_player.clone()).bytes(),
//                 );
//                 // Broadcast old players to new player
//                 for old_player in lobby.players.values() {
//                     server.send_message(
//                         *client_id,
//                         DefaultChannel::ReliableUnordered,
//                         MultiplayerEvent::PlayerCreated(old_player.clone()).bytes(),
//                     );
//                 }
//                 // Update local lobby
//                 lobby.players.insert(client_id.raw(), new_player);
//             }
//             ServerEvent::ClientDisconnected { client_id, reason } => {
//                 println!("Client {client_id} disconnected: {reason}");

//                 server.broadcast_message_except(
//                     *client_id,
//                     DefaultChannel::ReliableUnordered,
//                     MultiplayerEvent::PlayerDeleted(client_id.raw()).bytes(),
//                 );
//                 lobby.players.remove(&client_id.raw());
//             }
//         }
//     }
// }

// fn receive_message_system(
//     mut server: ResMut<RenetServer>,
//     mut events: EventWriter<MultiplayerEvent>,
// ) {
//     for client_id in server.clients_id() {
//         while let Some(msg) = server.receive_message(client_id, DefaultChannel::Unreliable) {
//             let message = bincode::deserialize(&msg).expect("Failed to deserialize message");
//             events.send(message);
//         }
//     }
// }

// fn sync_player_positions(
//     mut lobby: ResMut<ServerLobby>,
//     mut events: EventReader<MultiplayerEvent>,
//     mut server: ResMut<RenetServer>,
// ) {
//     for evt in events.read() {
//         let MultiplayerEvent::PlayerMoved(player) = evt else {
//             continue;
//         };
//         let Some(entry) = lobby.players.get_mut(&player.client_id) else {
//             continue;
//         };
//         entry.position = player.position;
//         server.broadcast_message_except(
//             ClientId::from_raw(player.client_id),
//             DefaultChannel::Unreliable,
//             MultiplayerEvent::PlayerMoved(player.clone()).bytes(),
//         );
//     }
// }
