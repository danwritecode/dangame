use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::{Instant, SystemTime},
};

use bincode::config::Configuration;
use common::types::{ClientEventType, ServerClient};
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};

use crate::characters::character::CharacterTrait;

pub struct ServerConnection {
    bincode_config: Configuration,
    client: RenetClient,
    transport: NetcodeClientTransport,
    last_renet_updated: Instant,
    last_server_updated: Instant,
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
        let last_renet_updated = Instant::now();
        let last_server_updated = Instant::now();
        let bincode_config = bincode::config::standard();
        let server_clients = HashMap::new();

        Self {
            bincode_config,
            client,
            transport,
            last_renet_updated,
            last_server_updated,
            client_id,
            server_clients,
        }
    }


    pub fn get_client_id(&self) -> u64 {
        self.client_id
    }

    pub fn get_last_server_updated(&self) -> Instant {
        self.last_server_updated
    }

    pub fn get_server_clients(&self) -> &HashMap<u64, ServerClient> {
        &self.server_clients
    }


    pub async fn handle_server_updates(&mut self) {
        let mut got_update = false;
        let now = Instant::now();
        let duration = now - self.last_renet_updated;
        self.last_renet_updated = now;

        self.client.update(duration);
        match self.transport.update(duration, &mut self.client) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error updating transport: {:?}", e);
            }
        }

        if self.client.is_connected() {
            while let Some(message) = self.client.receive_message(DefaultChannel::ReliableOrdered) {
                let (client_event_type, _len): (ClientEventType, usize) =
                    bincode::decode_from_slice(&message[..], self.bincode_config).unwrap();

                match client_event_type {
                    ClientEventType::ClientCharacterUpdate(message) => {
                        for (client_id, cse) in message {
                            self.server_clients
                                .entry(client_id)
                                .and_modify(|v| {
                                    v.prev_x_pos = v.x_pos;
                                    v.prev_y_pos = v.y_pos;
                                    v.x_pos = cse.x_pos;
                                    v.y_pos = cse.y_pos;
                                    v.height = cse.height;
                                    v.width = cse.width;
                                    v.anim_type = cse.anim_type.clone();
                                    v.character_type = cse.character_type.clone();
                                    v.sprite_frame = cse.sprite_frame;
                                    v.facing = cse.facing.clone();
                                })
                                .or_insert(cse);
                        }
                        got_update = true;
                    }
                }
            }
        }

        if got_update {
            let now = Instant::now();
            self.last_server_updated = now;
        }

        match self.transport.send_packets(&mut self.client) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error sending packets: {:?}", e);
            }
        }
    }


    pub async fn handle_client_updates(&mut self, my_character: &Box<dyn CharacterTrait>) {
        if self.client.is_connected() {
            let client_id = my_character.get_client_id().expect("Client ID not set");

            match self.server_clients.get_mut(&client_id) {
                Some(sc) => {
                    let size = my_character.get_size();
                    let pos = my_character.get_position();

                    sc.x_pos = pos.x;
                    sc.y_pos = pos.y;
                    sc.height = size.1; // 1 is height
                    sc.width = size.0; // 0 is width
                    sc.facing = my_character.get_facing();
                    sc.anim_type = my_character.get_anim_type();
                    sc.character_type = my_character.get_character_type();
                    sc.sprite_frame = my_character.get_sprite_frame();
                },
                None => {
                    eprintln!("Client ID not found in server clients: {:?}", client_id);
                }
            }

            match self.server_clients.get(&client_id) {
                Some(sc) => {
                    let encoded_client_server_event = bincode::encode_to_vec(sc, self.bincode_config).unwrap();
                    self.client.send_message(DefaultChannel::ReliableOrdered, encoded_client_server_event);
                },
                None => {
                    eprintln!("Client ID not found in server clients: {:?}", client_id);
                }
            }

        }

        match self.transport.send_packets(&mut self.client) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error sending packets: {:?}", e);
            }
        }
    }
}
