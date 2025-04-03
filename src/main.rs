use std::collections::HashSet;

use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};

struct Entity {
    textures: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_solid: bool
}

struct PlayerAnimation {
    name: String,
    row: usize,
    frames: usize,
    fps: usize,
    texture: Texture2D
}

enum PlayerState {
    Idle,
    Run,
    Jump,
}

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32,
    state: PlayerState,
    idle_anim: PlayerAnimation,
    run_anim: PlayerAnimation,
    jump_anim: PlayerAnimation
}

impl Player {
    async fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();

        let idle_anim = PlayerAnimation { name: "idle".to_string(), row: 0, frames: 6, fps: 20, texture: idle_texture };
        let run_anim = PlayerAnimation { name: "run".to_string(), row: 0, frames: 8, fps: 20, texture: run_texture };
        let jump_anim = PlayerAnimation { name: "jump".to_string(), row: 0, frames: 10, fps: 20, texture: jump_texture };

        let state = PlayerState::Idle;

        Self {
            x,
            y,
            width,
            height,
            speed,
            state,
            idle_anim,
            run_anim,
            jump_anim
        }
    }

    fn update(&mut self, dt: f32, keys_down: &HashSet<KeyCode>, entities: &Vec<Entity>) {
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    }
}

#[macroquad::main("Dangame")]
async fn main() {

    let mut p1 = Player::new(100.0, 100.0, 20.0, 20.0, 10.0).await;
    let mut entities:Vec<Entity> = Vec::new();

    loop {
        let dt = get_frame_time();
        let keys_down = get_keys_down();

        p1.update(dt, &keys_down, &entities);

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
