use std::{
    collections::{HashMap, HashSet}, net::{SocketAddr, UdpSocket}, thread, time::{Duration, Instant, SystemTime}
};

use common::types::{ClientEventType, ClientId, UserNameText, ClientServerEvent};
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{
    NetcodeServerTransport, ServerAuthentication, ServerConfig, NETCODE_USER_DATA_BYTES,
};


//
// full exmaple here
// https://github.com/lucaspoffo/renet/blob/master/renet/examples/echo.rs
//

fn main() {
    let server_addr: SocketAddr = format!("0.0.0.0:{}", 5000).parse().unwrap();
    server(server_addr);
}

const PROTOCOL_ID: u64 = 7;

fn server(public_addr: SocketAddr) {
    // bincode config
    let config = bincode::config::standard();

    let connection_config = ConnectionConfig::default();
    let mut server: RenetServer = RenetServer::new(connection_config);

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();

    let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    let mut clients: HashSet<ClientId> = HashSet::new();
    let mut client_states: HashMap<ClientId, ClientServerEvent> = HashMap::new();
    let mut client_mapping: HashMap<UserNameText, ClientId> = HashMap::new();

    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    clients.insert(client_id);

                    let user_data = transport.user_data(client_id).unwrap();
                    let username = Username::from_user_data(&user_data);
                    client_mapping.insert(username.0.clone(), client_id);

                    let client_mapping_event = ClientEventType::NewClient(client_mapping.clone());
                    let encoded_client_mapping_event = bincode::encode_to_vec(&client_mapping_event, config).unwrap();

                    server.broadcast_message_except(
                        client_id, 
                        DefaultChannel::ReliableOrdered, 
                        encoded_client_mapping_event
                    );

                    println!("Client {} connected. Username: {}", client_id, username.0);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }

        // this is where we get client updates
        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
                let (decoded, _len): (ClientServerEvent, usize) = bincode::decode_from_slice(&message[..], config).unwrap();
                let client_id = decoded.client_id;

                // insert or update client states
                client_states.entry(client_id)
                    .and_modify(|v| *v = decoded.clone())
                    .or_insert(decoded);
            }
        }

        // for each iteration of the loop, we send the client_states to all clients
        let client_mapping_event = ClientEventType::ClientCharacterUpdate(client_states.clone());
        let encoded_client_mapping_event = bincode::encode_to_vec(&client_mapping_event, config).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, encoded_client_mapping_event);

        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(50));
    }
}


struct Username(String);

impl Username {
    fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&user_data[0..8]);
        let mut len = u64::from_le_bytes(buffer) as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 8);
        let data = user_data[8..len + 8].to_vec();
        let username = String::from_utf8(data).unwrap();
        Self(username)
    }
}
