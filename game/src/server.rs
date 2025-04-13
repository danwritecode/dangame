use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::{Instant, SystemTime},
};

use bincode::config::Configuration;
use common::types::{ClientEventType, ServerClient};
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NETCODE_USER_DATA_BYTES, NetcodeClientTransport};

use crate::characters::character::{CharacterTrait, into_client_server_event};

pub struct ServerConnection {
    bincode_config: Configuration,
    client: RenetClient,
    transport: NetcodeClientTransport,
    last_updated: Instant,
    client_id: u64,
    server_clients: HashMap<u64, ServerClient>,
}

impl ServerConnection {
    pub fn new(server_addr: &str) -> Self {
        const PROTOCOL_ID: u64 = 7;
        let server_addr: SocketAddr = server_addr.parse().unwrap();

        let config = ConnectionConfig::default();
        let client = RenetClient::new(config.clone());
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;

        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id,
            user_data: None,
            protocol_id: PROTOCOL_ID,
        };

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        let last_updated = Instant::now();
        let bincode_config = bincode::config::standard();

        let server_characters = HashMap::new();

        Self {
            bincode_config,
            client,
            transport,
            last_updated,
            client_id,
            server_clients: server_characters,
        }
    }

    pub fn get_server_clients(&self) -> &HashMap<u64, ServerClient> {
        &self.server_clients
    }

    pub async fn handle_server_updates(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.client.update(duration);
        self.transport.update(duration, &mut self.client).unwrap();

        if self.client.is_connected() {
            while let Some(message) = self.client.receive_message(DefaultChannel::ReliableOrdered) {
                let (client_event_type, _len): (ClientEventType, usize) =
                    bincode::decode_from_slice(&message[..], self.bincode_config).unwrap();

                match client_event_type {
                    ClientEventType::ClientCharacterUpdate(message) => {
                        for (client_id, client_server_event) in message {
                            if client_id == self.client_id { continue; }

                            self.server_clients
                                .entry(client_id)
                                .and_modify(|v| {
                                    v.x_pos = client_server_event.x_pos;
                                    v.y_pos = client_server_event.y_pos;
                                    v.anim_type = client_server_event.anim_type.clone();
                                    v.character_type = client_server_event.character_type.clone();
                                    v.sprite_frame = client_server_event.sprite_frame;
                                    v.facing = client_server_event.facing.clone();
                                })
                                .or_insert(client_server_event);
                        }
                    }
                }
            }
        }

        self.transport.send_packets(&mut self.client).unwrap();
    }

    pub async fn handle_client_updates(&mut self, my_character: &Box<dyn CharacterTrait>) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.client.update(duration);
        self.transport.update(duration, &mut self.client).unwrap();

        if self.client.is_connected() {
            let client_server_event = into_client_server_event(my_character).await;
            let encoded_client_server_event =
                bincode::encode_to_vec(&client_server_event, self.bincode_config).unwrap();

            self.client.send_message(DefaultChannel::ReliableOrdered, encoded_client_server_event);
        }

        self.transport.send_packets(&mut self.client).unwrap();
    }
}
