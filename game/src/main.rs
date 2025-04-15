use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant, vec};

use characters::{character::CharacterTrait, character_1::Character1, character_2::Character2, character_3::Character3, server_character::ServerCharacter};
use common::{animation::{CharacterTextures, Facing}, types::ServerClient};

use macroquad::prelude::*;
use macroquad_tiled::{self as tiled, Map};
use macroquad_platformer::*;
use server::ServerConnection;


use common::constants::*;
use maps::map::GameMap;
use ui::main_menu::{MenuState, CharacterSelection};

mod characters;
mod maps;
mod types;
mod ui;
mod server;

enum GameState<'a> {
    Menu,
    Game(&'a GameMap),
}

const USE_HITBOXES: bool = false;
// this is how often we send client updates to the server
const CLIENT_UPDATE_INTERVAL_SECONDS: f32 = 0.01;
// this is how often the server sends updates to the clients
const SERVER_UPDATE_FREQUENCY_SECONDS: f32 = 0.016;

#[macroquad::main(window_conf)]
async fn main() {
    let mut server: Option<ServerConnection> = None;

    // TEXTURES AND WORLD BUILDING
    let character_textures = Rc::new(CharacterTextures::load_all().await);
    let splash_background = load_texture("assets/spritesheets/splash.png").await.unwrap();
    let tiled_map = load_map().await;
    let static_colliders = load_static_colliders(&tiled_map).await;

    let world = Rc::new(RefCell::new(World::new()));
    world.borrow_mut().add_static_tiled_layer(static_colliders, 32., 32., 40, 1);

    // Default my character to be Character1
    let mut my_character: Box<dyn CharacterTrait> = 
        Box::new(Character1::new(300.0, 50.0, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(&world), None).await);

    // only used in multiplayer
    let mut server_characters: HashMap<u64, ServerCharacter> = HashMap::new();

    let maps = get_maps().await;
    let mut game_state = GameState::Menu;
    let mut menu_state = MenuState::new();
    let mut is_multiplayer = false;
    let mut client_server_update_timer = 0.0;

    loop {
        let dt = get_frame_time();
        client_server_update_timer += dt;

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

                            // connect to server and set up client variables
                            if menu_state.connect_pressed {
                                server = Some(ServerConnection::new(&menu_state.server_address));
                                is_multiplayer = true;

                                if let Some(server) = &server {
                                    let client_id = server.get_client_id();
                                    add_my_character(character, &mut my_character, &world, 300.0, 50.0, Some(client_id)).await;
                                }
                            } else {
                                add_my_character(character, &mut my_character, &world, 300.0, 50.0, None).await;
                            }
                        }


                        game_state = GameState::Game(map);
                    }
                }
            }
            GameState::Game(map) => {
                map.draw_map();

                // draw update MY character
                render_update_my_character(my_character.as_mut(), dt, &world, &character_textures);
                
                if is_multiplayer {
                    // check if we have ALL the variables needed
                    if let Some(server) = server.as_mut() {
                        server.handle_server_updates().await;

                        // we don't want to send updates every frame
                        if client_server_update_timer >= CLIENT_UPDATE_INTERVAL_SECONDS {
                            server.handle_client_updates(&my_character).await;

                            // reset our time
                            client_server_update_timer = 0.0;
                        }

                        // Now we need to RENDER the server characters
                        let server_clients = server.get_server_clients();
                        
                        let now = Instant::now();
                        let duration = now - server.get_last_server_updated();
                        let t = (duration.as_secs_f32() / SERVER_UPDATE_FREQUENCY_SECONDS).clamp(0.0, 1.0);

                        render_update_server_characters(
                            server.get_client_id(), 
                            &server_clients, 
                            &mut server_characters, 
                            &character_textures, 
                            &world, 
                            t
                        ).await;
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
    y_pos: f32,
    client_id: Option<u64>
) {
    match character_selection {
        CharacterSelection::Character1 => {
            *my_character = Box::new(Character1::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world), client_id).await);
        },
        CharacterSelection::Character2 => {
            *my_character = Box::new(Character2::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world), client_id).await);
        },
        CharacterSelection::Character3 => {
            *my_character = Box::new(Character3::new(x_pos, y_pos, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, Rc::clone(world), client_id).await);
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

async fn render_update_server_characters(
    my_client_id: u64,
    server_clients: &HashMap<u64, ServerClient>,
    server_characters: &mut HashMap<u64, ServerCharacter>,
    textures: &Rc<CharacterTextures>,
    world: &Rc<RefCell<World>>,
    t: f32,
) {
    for (client_id, sc) in server_clients {
        if *client_id == my_client_id { continue; }

        // we calcualte lerp here
        let lerp_x_pos = lerp(sc.prev_x_pos, sc.x_pos, t);
        let lerp_y_pos = lerp(sc.prev_y_pos, sc.y_pos, t);

        let character = server_characters.entry(*client_id)
            .and_modify(|v| {
                v.x_pos = lerp_x_pos;
                v.y_pos = lerp_y_pos;
                v.height = sc.height;
                v.width = sc.width;
                v.anim_type = sc.anim_type.clone();
                v.character_type = sc.character_type.clone();
                v.sprite_frame = sc.sprite_frame;
                v.facing = sc.facing.clone();
            })
            .or_insert(ServerCharacter::new(
                lerp_x_pos,
                lerp_y_pos, 
                sc.height,
                sc.width, 
                sc.facing.clone(),
                sc.anim_type.clone(), 
                sc.character_type.clone(), 
                sc.sprite_frame, 
                Rc::clone(&world)
            ).await);

        let texture = character.get_texture(textures);
        let actor = character.get_actor();
        let facing = character.get_facing();
        let sprite_frame = character.get_sprite_frame();

        character.update();
        draw_player(&texture, &Rc::clone(&world), actor, facing, sprite_frame, USE_HITBOXES);
    }
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

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
