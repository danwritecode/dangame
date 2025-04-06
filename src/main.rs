use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};

use assets::{AnimationType, PlayerAnimation, AnimationBank};

mod assets;

const RUN_SPEED: f32 = 300.0;
const WALK_SPEED: f32 = 150.0;
const JUMP_SPEED: f32 = 400.0;
const GRAVITY: f32 = 800.0;
const FLOOR: f32 = 900.0;


struct Entity {
    textures: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_solid: bool
}

struct Player {
    x: f32,
    y: f32,
    x_v: f32,
    y_v: f32,
    facing: Facing,
    width: f32,
    height: f32,
    state: Rc<RefCell<PlayerAnimation>>,
    animation_bank: AnimationBank
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
    ) -> Self {
        let animation_bank = AnimationBank::load().await;
        let state = animation_bank.idle_anim.clone();

        Self {
            x,
            y,
            x_v: 0.0,
            y_v: 0.0,
            width,
            height,
            facing: Facing::Right,
            state,
            animation_bank
        }
    }

    fn update(&mut self, dt: f32, entities: &Vec<Entity>) {
        let wants_walk_left = is_key_down(KeyCode::A);
        let wants_walk_right = is_key_down(KeyCode::D);

        let wants_run_left = is_key_down(KeyCode::A) && is_key_down(KeyCode::LeftShift);
        let wants_run_right = is_key_down(KeyCode::D) && is_key_down(KeyCode::LeftShift);

        let wants_jump = is_key_down(KeyCode::Space);
        let wants_nothing = !is_any_key_down();

        let wants_attack_1 = is_key_down(KeyCode::E);
        let wants_attack_2 = is_key_down(KeyCode::Q);
        let wants_attack_3 = is_key_down(KeyCode::R);

        let mut next_animation_state = self.state.borrow().anim_type.clone();
        let is_airborn = self.y < FLOOR - self.height;
        let is_grounded = self.y >= FLOOR - self.height;
        let is_actively_playing = self.state.borrow().actively_playing;

        // if y is above floor, we are jumping
        if is_airborn {
            self.y_v -= GRAVITY * dt;
            self.y -=  self.y_v * dt;
        }

        if is_grounded { 
            self.y = FLOOR - self.height; 
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

        // apply velocity to x/y position after above calculations
        self.x += self.x_v * dt;
        self.y -= self.y_v * dt;

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

#[macroquad::main("Dangame")]
async fn main() {
    let background = load_texture("spritesheets/background_tokyo.png").await.unwrap();
    let mut p1 = Player::new(100.0, FLOOR - 20.0, 20.0, 80.0).await;
    let mut entities:Vec<Entity> = Vec::new();

    loop {
        let dt = get_frame_time();
        p1.update(dt, &entities);

        draw_texture(&background, 0., 0., WHITE);

        const TILE_WIDTH: f32 = 128.0;
        const TILE_HEIGHT: f32 = 128.0;
        const SPRITE_SHEET_ROW: u32 = 0;

        draw_texture_ex(
            &p1.state.borrow().texture,
            p1.x - 50.0,
            p1.y - 50.0,
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

        draw_rectangle(0.0, FLOOR, screen_width(), 300.0, BLACK);

        // draw some debugging text with player velocity
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(&format!("vx: {} | vy: {}", p1.x_v, p1.y_v), 20.0, 35.0, 20.0, DARKGRAY);
        draw_text(&format!("x: {} | y: {}", p1.x, p1.y), 20.0, 50.0, 20.0, DARKGRAY);
        draw_text(&format!("animation: {:?}", p1.state.borrow().anim_type), 20.0, 65.0, 20.0, DARKGRAY);
        draw_text(&format!("{:?}", FLOOR - p1.height), 20.0, 80.0, 20.0, DARKGRAY);

        // i want to see the animation frame data
        draw_text(&format!("sprite frame: {:?}", p1.state.borrow().sprite_frame), 20.0, 100.0, 20.0, DARKGRAY);
        draw_text(&format!("sequence frame:{:?}", p1.state.borrow().sequence_frame_index), 20.0, 115.0, 20.0, DARKGRAY);
        draw_text(&format!("sequence: {:?}", p1.state.borrow().sequence_index), 20.0, 130.0, 20.0, DARKGRAY);

        let (dx, dy) = p1.state.borrow_mut().update();

        // delta can be positive or negative so we always need to add it to the position
        p1.x += dx;
        p1.y += dy;

        next_frame().await
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
