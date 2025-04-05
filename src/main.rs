use macroquad::prelude::*;

use assets::Assets;

mod assets;

const RUN_SPEED: f32 = 300.0;
const JUMP_SPEED: f32 = 400.0;
const GRAVITY: f32 = 800.0;
const FLOOR: f32 = 500.0;


struct Entity {
    textures: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    is_solid: bool
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum AnimationType {
    Idle,
    ForwardRun,
    ReverseRun,
    Jump,
}

#[derive(Clone, Debug)]
struct PlayerAnimation {
    anim_type: AnimationType,
    sprite_frames: usize,
    anim_frames: usize,
    fps: usize,
    texture: Texture2D
}

struct Player<'a> {
    x: f32,
    y: f32,
    x_v: f32,
    y_v: f32,
    width: f32,
    height: f32,
    state: &'a PlayerAnimation,
    assets: &'a Assets,
    color: Color,
}

impl<'a> Player<'a> {
    async fn new(x: f32, y: f32, width: f32, height: f32, assets: &'a Assets) -> Self {
        Self {
            x,
            y,
            x_v: 0.0,
            y_v: 0.0,
            width,
            height,
            state: &assets.idle_anim,
            assets,
            color: GREEN
        }
    }

    fn update(&mut self, dt: f32, entities: &Vec<Entity>) {
        let wants_left = is_key_down(KeyCode::A);
        let wants_right = is_key_down(KeyCode::D);
        let wants_jump = is_key_down(KeyCode::Space);
        let wants_nothing = !is_any_key_down();

        let mut next_animation_state = &self.state.anim_type;

        // if we have velocity, we need to apply gravity
        if self.y < FLOOR - self.height {
            self.y_v -= GRAVITY * dt;
            self.y -=  self.y_v * dt;
        }

        if self.y >= FLOOR - self.height { 
            self.y = FLOOR - self.height; 
            self.y_v = 0.0; 
            next_animation_state = &AnimationType::Idle;
        }

        if wants_left {
            self.x_v = RUN_SPEED * -1.0;
            next_animation_state = &AnimationType::ReverseRun;

        }

        if wants_right {
            self.x_v = RUN_SPEED;
            next_animation_state = &AnimationType::ForwardRun;
        }

        if wants_jump {
            if self.y >= FLOOR - self.height {
                self.y_v = JUMP_SPEED;
                next_animation_state = &AnimationType::Jump;
            }
        }

        if wants_nothing {
            if self.y >= FLOOR {
                next_animation_state = &AnimationType::Idle;
            }
        }

        // apply velocity to x/y position after above calculations
        self.x += self.x_v * dt;
        self.y -= self.y_v * dt;

        // x velocity is only used for current frame
        self.x_v = 0.0;

        if next_animation_state != &self.state.anim_type { 
            match next_animation_state {
                AnimationType::Idle => {
                    self.state = &self.assets.idle_anim;
                },
                AnimationType::ForwardRun => {
                    self.state = &self.assets.fwd_run_anim;
                },
                AnimationType::ReverseRun => {
                    self.state = &self.assets.rev_run_anim;
                },
                AnimationType::Jump => {
                    self.state = &self.assets.jump_anim;
                }
            }
        }

        match self.state.anim_type {
            AnimationType::Idle => {
                self.color = GREEN;
            },
            AnimationType::ForwardRun => {
                self.color = ORANGE;
            },
            AnimationType::ReverseRun => {
                self.color = ORANGE;
            },
            AnimationType::Jump => {
                self.color = RED;
            }
        }
    }
}

#[macroquad::main("Dangame")]
async fn main() {
    let assets = Assets::load().await;

    let mut p1 = Player::new(100.0, FLOOR - 20.0, 20.0, 20.0, &assets).await;
    let mut entities:Vec<Entity> = Vec::new();

    loop {
        let dt = get_frame_time();

        p1.update(dt, &entities);

        // draw some debugging text with player velocity
        draw_text(&format!("vx: {} | vy: {}", p1.x_v, p1.y_v), 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(&format!("x: {} | y: {}", p1.x, p1.y), 20.0, 35.0, 20.0, DARKGRAY);
        draw_text(&format!("animation: {:?}", p1.state.anim_type), 20.0, 50.0, 20.0, DARKGRAY);
        draw_text(&format!("{:?}", FLOOR - p1.height), 20.0, 70.0, 20.0, DARKGRAY);

        // we are building a platformer, so there are no Y axis movement (unless jumping)
        draw_rectangle(p1.x, p1.y, p1.width, p1.height, p1.color);
        draw_line(0.0, FLOOR, screen_width(), FLOOR, 2.0, RED);


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
