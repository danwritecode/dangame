use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

use characters::{character::CharacterTrait, character_1::Character1, character_2::Character2, server_character::ServerCharacter};
use common::{animation::{CharacterTextures, Facing}, types::{ClientEventType, ClientServerEvent, UserNameText}};

use macroquad::prelude::*;
use macroquad_tiled::{self as tiled, Map};
use macroquad_platformer::*;


// renet stuff
use std::{
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant, SystemTime},
};

use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{
    ClientAuthentication, NetcodeClientTransport, NETCODE_USER_DATA_BYTES
};


use constants::*;
use maps::map::GameMap;
use ui::main_menu::{MenuState, CharacterSelection};

mod characters;
mod constants;
mod maps;
mod types;
mod ui;


enum GameState<'a> {
    Menu,
    Game(&'a GameMap),
}

const USE_HITBOXES: bool = true;

#[macroquad::main(window_conf)]
async fn main() {
    // CLIENT STUFF
    let config = bincode::config::standard();
    let mut client: Option<RenetClient> = None;
    let mut transport: Option<NetcodeClientTransport> = None;
    let mut last_updated: Option<Instant> = None;
    let username = format!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());


    // TEXTURES AND WORLD BUILDING
    let character_textures = Rc::new(CharacterTextures::load_all().await);
    let splash_background = load_texture("assets/spritesheets/splash.png").await.unwrap();
    let tiled_map = load_map().await;
    let static_colliders = load_static_colliders(&tiled_map).await;

    let world = Rc::new(RefCell::new(World::new()));
    world.borrow_mut().add_static_tiled_layer(static_colliders, 32., 32., 40, 1);

    // Default my character to be Character1
    // it'll be overwritten once character is selected
    let mut my_character: Box<dyn CharacterTrait> = 
        Box::new(Character1::new(300.0, 50.0, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(&world)).await);

    // only used in multiplayer
    let mut other_characters: HashMap<UserNameText, ServerCharacter> = HashMap::new();

    let maps = get_maps().await;
    let mut game_state = GameState::Menu;
    let mut menu_state = MenuState::new();
    let mut is_multiplayer = false;

    loop {
        let dt = get_frame_time();

        match game_state {
            GameState::Menu => {
                clear_background(BLACK);

                let callback = Rc::new(RefCell::new(|i: usize| { 
                    let map = maps.get(i).unwrap();
                    game_state = GameState::Game(map); 
                }));

                ui::main_menu::draw_menu(&splash_background, &maps, &mut menu_state, callback).await;
                
                // Check if we need to transition to the game
                if let Some(map_index) = menu_state.map_selection {
                    if let Some(map) = maps.get(map_index) {
                        if let Some(character) = &menu_state.character_selection {
                            add_my_character(character, &mut my_character, &world, 300.0, 50.0).await;
                        }
                        
                        is_multiplayer = menu_state.connect_pressed;

                        // connect to server and set up client variables
                        if is_multiplayer {
                            let server_connection_response = connect_to_server(&username).await;
                            client = Some(server_connection_response.client);
                            transport = Some(server_connection_response.transport);
                            last_updated = Some(server_connection_response.last_updated);
                        }

                        game_state = GameState::Game(map);
                    }
                }
            }
            GameState::Game(map) => {
                map.draw_map();

                // draw the other characters
                for (k, character) in other_characters.iter_mut() {
                    // render_character(character.as_mut(), &world, &character_textures, use_hitboxes);
                }

                // draw AND update my character
                render_update_my_character(my_character.as_mut(), dt, &world, &character_textures);
                
                // Display multiplayer indicator if we're in multiplayer mode
                if is_multiplayer {
                    // check if we have ALL the variables needed
                    match (
                        client.as_mut(),
                        transport.as_mut(),
                        last_updated.as_mut(),
                    ) {
                        (Some(client), Some(transport), Some(last_updated)) => {
                            handle_client_updates(client, config, transport, last_updated, &username);
                        }
                        _ => (),
                    }
                }
            }
        }

        next_frame().await
    }
}


async fn add_my_character(
    character_selection: &CharacterSelection, 
    my_character: &mut Box<dyn CharacterTrait>, 
    world: &Rc<RefCell<World>>, 
    x_pos: f32, 
    y_pos: f32
) {
    match character_selection {
        CharacterSelection::Character1 => {
            *my_character = Box::new(Character1::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world)).await);
        },
        CharacterSelection::Character2 => {
            *my_character = Box::new(Character2::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world)).await);
        },
        CharacterSelection::Character3 => {
            *my_character = Box::new(Character1::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world)).await);
        },
    }
}


async fn get_maps() -> Vec<GameMap> {
    vec![
        GameMap::new(
            "First Map".to_owned(),
            "assets/spritesheets/bg_night_tokyo.png".to_owned(),
            "assets/maps/tilesets/exclusion-zone-tileset/1 Tiles/Tileset.png".to_owned(),
            "assets/maps/map_01.json".to_owned(),
            "Tileset.png".to_owned(),
            vec!["Platforms".to_owned()],
        ).await,
    ]
}

fn render_update_my_character(
    character: &mut dyn CharacterTrait, 
    dt: f32, 
    world: &Rc<RefCell<World>>, 
    textures: &Rc<CharacterTextures>,
) {
    character.update(dt);
    let texture = character.get_texture(textures);
    let actor = character.get_actor();
    let facing = character.get_facing();
    let sprite_frame = character.get_sprite_frame();

    draw_player(&texture, &Rc::clone(&world), actor, facing, sprite_frame, USE_HITBOXES);
}

fn draw_player(
    texture: &Texture2D,
    world: &Rc<RefCell<World>>,
    actor: Actor,
    facing: Facing,
    sprite_frame: usize,
    draw_hitboxes: bool
) {
    let player_pos = world.borrow_mut().actor_pos(actor);
    let player_size = world.borrow_mut().actor_size(actor);

    if draw_hitboxes {
        draw_rectangle_lines(player_pos.x, player_pos.y, player_size.0 as f32,  player_size.1 as f32, 4.0, RED);
    }

    draw_texture_ex(
        texture,
        player_pos.x - (SPRITE_WIDTH / 2.0) + (player_size.0 as f32 / 2.0),
        player_pos.y - (SPRITE_HEIGHT - player_size.1 as f32),
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                TILE_WIDTH * sprite_frame as f32,
                TILE_HEIGHT * SPRITE_SHEET_ROW as f32,
                TILE_WIDTH,
                TILE_HEIGHT,
            )),
            dest_size: Some(vec2(TILE_WIDTH, TILE_HEIGHT)),
            flip_x: facing == Facing::Left,
            ..Default::default()
        }
    );
}

async fn load_map() -> Map {
    let tileset = load_texture("assets/maps/tilesets/exclusion-zone-tileset/1 Tiles/Tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/maps/map_01.json").await.unwrap();
    tiled::load_map(
        &tiled_map_json,
        &[("Tileset.png", tileset)],
        &[]
    ).unwrap()
}

async fn load_static_colliders(tiled_map: &Map) -> Vec<Tile> { 
    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("Platforms", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    static_colliders
}

fn window_conf() -> Conf {
    Conf {
        window_title: "dangame".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

async fn handle_client_updates(
    client: &mut RenetClient,
    bincode_config: bincode::config::Configuration,
    transport: &mut NetcodeClientTransport,
    last_updated: &mut Instant,
    client_username: &str,
    characters: &mut HashMap<UserNameText, ServerCharacter>,
    world: &Rc<RefCell<World>>,
    textures: &Rc<CharacterTextures>,
) {
    let now = Instant::now();
    let duration = now - *last_updated;
    *last_updated = now;

    client.update(duration);
    transport.update(duration, client).unwrap();

    if client.is_connected() {

        // this is where we send the updated client info
        // let client_mapping_event = ClientEventType::ClientCharacterUpdate(client_states.clone());
        // let encoded_client_mapping_event = bincode::encode_to_vec(&client_mapping_event, config).unwrap();
        // client.send_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec());


        while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
            let (client_event_type, _len): (ClientEventType, usize) = bincode::decode_from_slice(&message[..], bincode_config).unwrap();

            match client_event_type {
                ClientEventType::ClientCharacterUpdate(message) => {
                    println!("Client Character Update");
                    // let character = Character1::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world)).await;

                    for (username, client_server_event) in message { 
                        if username == client_username { continue; } // skip our own character

                        let x_pos = client_server_event.x_pos;
                        let y_pos = client_server_event.y_pos;

                        // update character or insert it
                        let character = characters.entry(username)
                            .and_modify(|v| {
                                v.x_v = x_pos;
                                v.y_v = y_pos;
                                v.anim_type = client_server_event.anim_type.clone();
                                v.character_type = client_server_event.character_type.clone();
                                v.sprite_frame = client_server_event.sprite_frame;
                            })
                            .or_insert(ServerCharacter::new(
                                x_pos, y_pos, 
                                DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, 
                                client_server_event.anim_type.clone(), 
                                client_server_event.character_type.clone(), 
                                client_server_event.sprite_frame, 
                                Rc::clone(&world)
                            ).await);

                        let texture = character.get_texture(textures);
                        let actor = character.get_actor();
                        let facing = character.get_facing();
                        let sprite_frame = character.get_sprite_frame();

                        draw_player(&texture, &Rc::clone(&world), actor, facing, sprite_frame, USE_HITBOXES);
                    }
                }
            }

        }
    }

    transport.send_packets(client).unwrap();
}

struct ServerConnectionResponse {
    client: RenetClient,
    transport: NetcodeClientTransport,
    last_updated: Instant,
}

async fn connect_to_server(username: &str) -> ServerConnectionResponse {
    const PROTOCOL_ID: u64 = 7;
    let server_addr: SocketAddr = "34.234.74.134:5000".parse().unwrap();

    let connection_config = ConnectionConfig::default();
    let client = RenetClient::new(connection_config);
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: Some(to_netcode_user_data(username)),
        protocol_id: PROTOCOL_ID,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let last_updated = Instant::now();

    ServerConnectionResponse {
        client,
        transport,
        last_updated,
    }
}

fn to_netcode_user_data(username: &str) -> [u8; NETCODE_USER_DATA_BYTES] {
    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too big");
    }
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    user_data
}
