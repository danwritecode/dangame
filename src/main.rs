use std::{cell::RefCell, rc::Rc};

use macroquad::{prelude::{animation::{AnimatedSprite, Animation}, *}, window};
use macroquad_tiled::{self as tiled, Map};
use macroquad_platformer::*;


use assets::{AnimationType, PlayerAnimation, AnimationBank};

mod assets;

const RUN_SPEED: f32 = 300.0;
const WALK_SPEED: f32 = 150.0;
const JUMP_SPEED: f32 = 400.0;
const GRAVITY: f32 = 800.0;

const WINDOW_HEIGHT: i32 = 832;
const WINDOW_WIDTH: i32 = 1280;

const PLAYER_WIDTH: f32 = 28.0;
const PLAYER_HEIGHT: f32 = 93.0;

const TILE_WIDTH: f32 = 128.0;
const TILE_HEIGHT: f32 = 128.0;
const SPRITE_SHEET_ROW: u32 = 0;

struct Player {
    x: f32,
    y: f32,
    x_v: f32,
    y_v: f32,
    facing: Facing,
    width: f32,
    height: f32,
    state: Rc<RefCell<PlayerAnimation>>,
    animation_bank: AnimationBank,
    collider: Actor,
    world: Rc<RefCell<World>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Facing {
    Left,
    Right,
}

impl Player {
    async fn new(
        x: f32, 
        y: f32, 
        width: f32, 
        height: f32, 
        world: Rc<RefCell<World>>,
    ) -> Self {
        let animation_bank = AnimationBank::load().await;
        let state = animation_bank.idle_anim.clone();
        let collider = world.borrow_mut().add_actor(vec2(x, y), width as i32, height as i32);

        Self {
            x,
            y,
            x_v: 0.0,
            y_v: 0.0,
            width,
            height,
            facing: Facing::Right,
            state,
            animation_bank,
            collider,
            world
        }
    }

    fn update(&mut self, dt: f32) {
        let wants_walk_left = is_key_down(KeyCode::A);
        let wants_walk_right = is_key_down(KeyCode::D);

        let wants_run_left = is_key_down(KeyCode::A) && is_key_down(KeyCode::LeftShift);
        let wants_run_right = is_key_down(KeyCode::D) && is_key_down(KeyCode::LeftShift);

        let wants_jump = is_key_down(KeyCode::Space);
        let wants_nothing = !is_any_key_down();

        let wants_attack_1 = is_key_down(KeyCode::E);
        let wants_attack_2 = is_key_down(KeyCode::Q);
        let wants_attack_3 = is_key_down(KeyCode::R);

        // NOT WORKING YET
        let pos = self.world.borrow().actor_pos(self.collider);
        let is_grounded = self.world.borrow().collide_check(self.collider, pos + vec2(0., 1.));

        let is_airborn = !is_grounded;
        let is_actively_playing = self.state.borrow().actively_playing;

        let mut next_animation_state = self.state.borrow().anim_type.clone();

        // if y is above floor, we are jumping
        if is_airborn {
            self.y_v -= GRAVITY * dt;
            self.y -=  self.y_v * dt;
        }

        if is_grounded { 
            // self.y = FLOOR - self.height; 
            self.y_v = 0.0; 
            next_animation_state = AnimationType::Idle;
        }

        if wants_walk_left {
            self.x_v = WALK_SPEED * -1.0;
            self.facing = Facing::Left;
            if is_grounded { next_animation_state = AnimationType::ReverseWalk; }
            
        }

        if wants_walk_right {
            self.x_v = WALK_SPEED;
            self.facing = Facing::Right;
            if is_grounded { next_animation_state = AnimationType::ForwardWalk; }
        }

        if wants_run_left {
            self.x_v = RUN_SPEED * -1.0;
            self.facing = Facing::Left;
            if is_grounded { next_animation_state = AnimationType::ReverseRun; }
            
        }

        if wants_run_right {
            self.x_v = RUN_SPEED;
            self.facing = Facing::Right;
            if is_grounded { next_animation_state = AnimationType::ForwardRun; }
        }

        if wants_jump {
            self.y_v = JUMP_SPEED;
            if is_grounded {
                self.y_v = JUMP_SPEED;
                next_animation_state = AnimationType::Jump;
            }
        }

        if wants_nothing {
            if is_grounded {
                next_animation_state = AnimationType::Idle;
            }
        }

        if wants_attack_1 {
            if is_grounded {
                next_animation_state = AnimationType::Attack1;
            }
        }

        if wants_attack_2 {
            if is_grounded {
                next_animation_state = AnimationType::Attack2;
            }
        }

        if wants_attack_3 {
            if is_grounded {
                next_animation_state = AnimationType::Attack3;
            }
        }

        self.world.borrow_mut().move_h(self.collider, self.x_v * dt);
        self.world.borrow_mut().move_v(self.collider, (self.y_v * -1.0) * dt);

        // x velocity is only used for current frame
        self.x_v = 0.0;

        if next_animation_state != self.state.borrow().anim_type && !is_actively_playing {
            // we decided above if we want to change animations or not
            // if we want to change animations, we need to stop the current animation
            // self.state.borrow_mut().reset();

            match next_animation_state {
                AnimationType::Idle => {
                    self.state = Rc::clone(&self.animation_bank.idle_anim);
                },
                AnimationType::ForwardRun  => {
                    self.state = Rc::clone(&self.animation_bank.fwd_run_anim);
                },
                AnimationType::ReverseRun  => {
                    self.state = Rc::clone(&self.animation_bank.rev_run_anim);
                },
                AnimationType::ForwardWalk  => {
                    self.state = Rc::clone(&self.animation_bank.fwd_walk_anim);
                },
                AnimationType::ReverseWalk  => {
                    self.state = Rc::clone(&self.animation_bank.rev_walk_anim);
                }
                AnimationType::Jump  => {
                    self.state = Rc::clone(&self.animation_bank.jump_anim);
                    self.state.borrow_mut().actively_playing = true;
                },
                AnimationType::Attack1 => {
                    self.state = Rc::clone(&self.animation_bank.attack_1_anim);
                    self.state.borrow_mut().actively_playing = true;
                },
                AnimationType::Attack2 => {
                    self.state = Rc::clone(&self.animation_bank.attack_2_anim);
                    self.state.borrow_mut().actively_playing = true;
                },
                AnimationType::Attack3 => {
                    self.state = Rc::clone(&self.animation_bank.attack_3_anim);
                    self.state.borrow_mut().actively_playing = true;
                },
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let background = load_texture("spritesheets/bg_night_tokyo.png").await.unwrap();
    let tiled_map = load_map().await;
    let static_colliders = load_static_colliders(&tiled_map).await;

    let mut world = Rc::new(RefCell::new(World::new()));
    world.borrow_mut().add_static_tiled_layer(static_colliders, 32., 32., 40, 1);

    let mut p1 = Player::new(600.0, 50.0, PLAYER_WIDTH, PLAYER_HEIGHT, world.clone()).await;

    loop {
        let dt = get_frame_time();
        draw_texture(&background, 0., 0., WHITE);
        tiled_map.draw_tiles("Platforms", Rect::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32), None);

        p1.update(dt);

        let pos = world.borrow_mut().actor_pos(p1.collider);
        // draw_rectangle_lines(pos.x, pos.y, PLAYER_WIDTH, PLAYER_HEIGHT, 4.0, RED);

        // player gets drawn here
        draw_texture_ex(
            &p1.state.borrow().texture,
            pos.x - PLAYER_WIDTH - 22.0,
            pos.y - 35.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    TILE_WIDTH * p1.state.borrow().sprite_frame as f32,
                    TILE_HEIGHT * SPRITE_SHEET_ROW as f32,
                    TILE_WIDTH,
                    TILE_HEIGHT,
                )),
                dest_size: Some(vec2(128.0, 128.0)),
                flip_x: p1.facing == Facing::Left,
                ..Default::default()
            }
        );

        // draw some debugging text with player velocity
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(&format!("vx: {} | vy: {}", p1.x_v, p1.y_v), 20.0, 35.0, 20.0, DARKGRAY);
        draw_text(&format!("x: {} | y: {}", p1.x, p1.y), 20.0, 50.0, 20.0, DARKGRAY);
        draw_text(&format!("animation: {:?}", p1.state.borrow().anim_type), 20.0, 65.0, 20.0, DARKGRAY);

        // i want to see the animation frame data
        draw_text(&format!("sprite frame: {:?}", p1.state.borrow().sprite_frame), 20.0, 100.0, 20.0, DARKGRAY);
        draw_text(&format!("sequence frame:{:?}", p1.state.borrow().sequence_frame_index), 20.0, 115.0, 20.0, DARKGRAY);
        draw_text(&format!("sequence: {:?}", p1.state.borrow().sequence_index), 20.0, 130.0, 20.0, DARKGRAY);

        let (dx, dy) = p1.state.borrow_mut().update();

        if p1.facing == Facing::Left {
            world.borrow_mut().move_h(p1.collider, dx * -1.0);
        } else {
            world.borrow_mut().move_h(p1.collider, dx);
        }
        world.borrow_mut().move_v(p1.collider, dy);

        next_frame().await
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

fn is_any_key_down() -> bool {
    if is_key_down(KeyCode::W) { return true; }
    if is_key_down(KeyCode::S) { return true; }
    if is_key_down(KeyCode::A) { return true; }
    if is_key_down(KeyCode::D) { return true; }
    if is_key_down(KeyCode::Space) { return true; }
    false
}
