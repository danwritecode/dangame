use std::{cell::RefCell, rc::Rc, vec};

use characters::{character_1::Character1, character_2::Character2};
use common::character::{CharacterTrait, Facing};
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
    ClientAuthentication, NetcodeClientTransport
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

#[macroquad::main(window_conf)]
async fn main() {
    let use_hitboxes = true;

    // let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    // client(server_addr);

    let splash_background = load_texture("spritesheets/splash.png").await.unwrap();
    let tiled_map = load_map().await;
    let static_colliders = load_static_colliders(&tiled_map).await;

    let world = Rc::new(RefCell::new(World::new()));
    world.borrow_mut().add_static_tiled_layer(static_colliders, 32., 32., 40, 1);

    // Start with an empty characters vector
    let mut characters: Box<Vec<Box<dyn CharacterTrait>>> = Box::new(vec![]);

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
                            add_character(character, &mut characters, &world, 300.0, 50.0).await;
                        }
                        
                        is_multiplayer = menu_state.connect_pressed;
                        game_state = GameState::Game(map);
                    }
                }
            }
            GameState::Game(map) => {
                map.draw_map();
                for character in characters.iter_mut() {
                    render_character(character.as_mut(), dt, &world, use_hitboxes);
                }
                
                // Display multiplayer indicator if we're in multiplayer mode
                if is_multiplayer {
                    draw_text("Multiplayer Mode", 20.0, 20.0, 20.0, GREEN);
                }
            }
        }

        next_frame().await
    }
}


/// Add a character to the characters vector based on the provided selection
async fn add_character(
    character_selection: &CharacterSelection,
    characters: &mut Vec<Box<dyn CharacterTrait>>,
    world: &Rc<RefCell<World>>,
    x_pos: f32,
    y_pos: f32,
) {
    match character_selection {
        CharacterSelection::Character1 => {
            let character = Character1::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, world.clone()).await;
            characters.push(Box::new(character));
        },
        CharacterSelection::Character2 => {
            let character = Character2::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, world.clone()).await;
            characters.push(Box::new(character));
        },
        CharacterSelection::Character3 => {
            // Placeholder for Character3 - using Character1 for now
            let character = Character1::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, world.clone()).await;
            characters.push(Box::new(character));
        },
    }
}


async fn get_maps() -> Vec<GameMap> {
    vec![
        GameMap::new(
            "First Map".to_owned(),
            "spritesheets/bg_night_tokyo.png".to_owned(),
            "tilesets/exclusion-zone-tileset/1 Tiles/Tileset.png".to_owned(),
            "maps/map_01.json".to_owned(),
            "Tileset.png".to_owned(),
            vec!["Platforms".to_owned()],
        ).await,
    ]
}

fn render_character(
    character: &mut dyn CharacterTrait, 
    dt: f32, world: 
    &Rc<RefCell<World>>, 
    use_hitboxes: bool
) {
    character.update(dt);
    let texture = character.get_texture();
    let actor = character.get_actor();
    let facing = character.get_facing();
    let sprite_frame = character.get_sprite_frame();

    draw_player(&texture, &Rc::clone(&world), actor, facing, sprite_frame, use_hitboxes);
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

fn draw_debug(
    actor: Actor,
    world: &Rc<RefCell<World>>,
    x_v: f32,
    y_v: f32,
    use_debug: bool
) {
    if use_debug {
        let player_pos = world.borrow_mut().actor_pos(actor);
        let player_size = world.borrow_mut().actor_size(actor);

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(&format!("vx: {} | vy: {}", x_v, y_v), 20.0, 35.0, 20.0, DARKGRAY);
        draw_text(&format!("x: {} | y: {}", player_pos.x, player_pos.y), 20.0, 50.0, 20.0, DARKGRAY);

        // player size
        draw_text(&format!("width: {} | height: {}", player_size.0, player_size.1), 20.0, 65.0, 20.0, DARKGRAY);
    }
}

async fn load_map() -> Map {
    let tileset = load_texture("tilesets/exclusion-zone-tileset/1 Tiles/Tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("maps/map_01.json").await.unwrap();
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

fn client(server_addr: SocketAddr) {
    const PROTOCOL_ID: u64 = 7;

    let connection_config = ConnectionConfig::default();
    let mut client = RenetClient::new(connection_config);

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let stdin_channel: Receiver<String> = spawn_stdin_channel();

    let mut last_updated = Instant::now();
    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        client.update(duration);
        transport.update(duration, &mut client).unwrap();

        if client.is_connected() {
            match stdin_channel.try_recv() {
                Ok(text) => client.send_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec()),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            while let Some(text) = client.receive_message(DefaultChannel::ReliableOrdered) {
                let text = String::from_utf8(text.into()).unwrap();
                println!("{}", text);
            }
        }

        transport.send_packets(&mut client).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer.trim_end().to_string()).unwrap();
    });
    rx
}
