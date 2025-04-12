use std::{
    collections::HashSet, net::{SocketAddr, UdpSocket}, thread, time::{Duration, Instant, SystemTime}
};

use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{
    NetcodeServerTransport, ServerAuthentication, ServerConfig,
};


//
// full exmaple here
// https://github.com/lucaspoffo/renet/blob/master/renet/examples/echo.rs
//


// Client joins server
// Server adds Character to Server's list of Characters
// Server sends all Characters to all clients
// Clients update their Characters


fn main() {
    let server_addr: SocketAddr = format!("0.0.0.0:{}", 5000).parse().unwrap();
    server(server_addr);
}

const PROTOCOL_ID: u64 = 7;

fn server(public_addr: SocketAddr) {
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

    let mut clients: HashSet<u64> = HashSet::new();
    let mut received_messages: Vec<String> = vec![];
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        received_messages.clear();

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    clients.insert(client_id);

                    server.broadcast_message_except(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        format!("User \"{}\" connected", client_id),
                    );

                    println!("Client {} connected.", client_id)
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
                // this is where we get client updates
                // and we have to push them to all other clients
                // create a vec to hold all messages
            }
        }

        for text in received_messages.iter() {
            // make this a broadcast message except
            server.broadcast_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec());
        }

        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(50));
    }
}
