use std::{
    collections::{HashMap, HashSet}, net::{SocketAddr, UdpSocket}, thread, time::{Duration, Instant, SystemTime}
};

use common::types::{ClientEventType, ServerClient};
use renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{
    NetcodeServerTransport, ServerAuthentication, ServerConfig,
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
    let mut client_states: HashMap<ClientId, ServerClient> = HashMap::new();
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        match transport.update(duration, &mut server) {
            Ok(_) => {},
            Err(e) => {
                println!("Error updating transport: {:?}", e);
                continue;
            }
        };

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected", client_id);
                    client_states.insert(client_id, ServerClient::default());
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    client_states.remove(&client_id);
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }

        // this is where we get client updates
        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
                let (decoded, _len): (ServerClient, usize) = match bincode::decode_from_slice(&message[..], config) {
                    Ok(decoded) => decoded,
                    Err(e) => {
                        println!("Error decoding message: {:?}", e);
                        continue;
                    }
                };

                // insert or update client states
                client_states.entry(client_id)
                    .and_modify(|v| *v = decoded.clone())
                    .or_insert(decoded.clone());
            }
        }


        let client_mapping_event = ClientEventType::ClientCharacterUpdate(client_states.clone());
        let encoded_client_mapping_event = match bincode::encode_to_vec(&client_mapping_event, config) {
            Ok(encoded_client_mapping_event) => encoded_client_mapping_event,
            Err(e) => {
                println!("Error encoding client mapping event: {:?}", e);
                continue;
            }
        };

        server.broadcast_message(DefaultChannel::ReliableOrdered, encoded_client_mapping_event);
        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(16));
    }
}
