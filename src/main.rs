use std::{cell::RefCell, rc::Rc, vec};

use macroquad::prelude::*;
use macroquad_tiled::{self as tiled, Map};
use macroquad_platformer::*;

use characters::{character_1::Character1, character_2::Character2, characters::{CharacterTrait, Facing}};

use constants::*;

mod characters;
mod constants;
mod types;



#[macroquad::main(window_conf)]
async fn main() {
    let use_hitboxes = true;

    let background = load_texture("spritesheets/bg_night_tokyo.png").await.unwrap();
    let tiled_map = load_map().await;
    let static_colliders = load_static_colliders(&tiled_map).await;

    let world = Rc::new(RefCell::new(World::new()));
    world.borrow_mut().add_static_tiled_layer(static_colliders, 32., 32., 40, 1);

    let mut c1 = Character1::new(600.0, 50.0, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, world.clone()).await;
    let mut c2 = Character2::new(100.0, 50.0, DEFAULT_PLAYER_WIDTH, DEFAULT_PLAYER_HEIGHT, world.clone()).await;

    loop {
        let dt = get_frame_time();
        draw_map(&tiled_map, &background);

        render_character(&mut c1, dt, &Rc::clone(&world), use_hitboxes);
        render_character(&mut c2, dt, &Rc::clone(&world), use_hitboxes);

        next_frame().await
    }
}

fn draw_map(tiled_map: &Map, background: &Texture2D) {
    draw_texture(background, 0., 0., WHITE);
    tiled_map.draw_tiles("Platforms", Rect::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32), None);
}


fn render_character<A: CharacterTrait>(character: &mut A, dt: f32, world: &Rc<RefCell<World>>, use_hitboxes: bool) {
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
