use std::collections::VecDeque;

use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};

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
    texture: Texture2D,
    x_v: f32,
    y_v: f32,
    x_v_decay: f32,
    y_v_decay: f32,
}

impl PlayerAnimation {
    fn execute(&mut self, dt: f32, player: &mut Player) {
        self.x_v -= self.x_v_decay * dt;
        self.y_v -= self.y_v_decay * dt;
    }
}

struct Player {
    x: f32,
    y: f32,
    x_v: f32,
    y_v: f32,
    width: f32,
    height: f32,
    cur_animation: PlayerAnimation,
    idle_anim: PlayerAnimation,
    fwd_run_anim: PlayerAnimation,
    rev_run_anim: PlayerAnimation,
    jump_anim: PlayerAnimation,
    color: Color
}

impl Player {
    async fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();

        let idle_anim = PlayerAnimation { anim_type: AnimationType::Idle, sprite_frames: 6, anim_frames: 6, fps: 20, 
            texture: idle_texture, x_v: 0.0, y_v: 0.0, x_v_decay: 0.0, y_v_decay: 0.0 };
        
        let fwd_run_anim = PlayerAnimation { anim_type: AnimationType::ForwardRun, sprite_frames: 8, anim_frames: 8, fps: 20, 
            texture: run_texture.clone(), x_v: 200.0, y_v: 0.0, x_v_decay: 200.0, y_v_decay: 0.0 };

        let rev_run_anim = PlayerAnimation { anim_type: AnimationType::ReverseRun, sprite_frames: 8, anim_frames: 8, fps: 20, 
            texture: run_texture, x_v: 200.0, y_v: 0.0, x_v_decay: 200.0, y_v_decay: 0.0 };

        let jump_anim = PlayerAnimation { anim_type: AnimationType::Jump, sprite_frames: 10, anim_frames: 10, fps: 20, 
            texture: jump_texture, x_v: 0.0, y_v: 400.0, x_v_decay: 200.0, y_v_decay: 200.0 };

        Self {
            x,
            y,
            x_v: 0.0,
            y_v: 0.0,
            width,
            height,
            cur_animation: idle_anim.clone(),
            idle_anim,
            fwd_run_anim,
            rev_run_anim,
            jump_anim,
            color: GREEN
        }
    }

    fn update(&mut self, dt: f32, entities: &Vec<Entity>) {
        let wants_left = is_key_down(KeyCode::A);
        let wants_right = is_key_down(KeyCode::D);
        let wants_jump = is_key_down(KeyCode::Space);
        let wants_nothing = !is_any_key_down();

        let mut next_animation = self.cur_animation.clone();

        if wants_left {
            next_animation = self.rev_run_anim.clone();
        }
        if wants_right {
            next_animation = self.fwd_run_anim.clone();
        }
        if wants_jump {
            // don't want to double jump
            if self.cur_animation.anim_type != AnimationType::Jump { 
                next_animation = self.jump_anim.clone();
            }
        }
        if wants_nothing {
            next_animation = self.idle_anim.clone();
        }



        if is_key_down(KeyCode::A) {
            match self.cur_animation.anim_type {
                AnimationType::ReverseRun => {
                    // we want to reset the velocity
                    self.x_v = self.cur_animation.x_v;
                    self.x_v = self.x_v * -1.0;
                },
                AnimationType::ForwardRun => {
                    // we want to reset the velocity
                    self.cur_animation = self.rev_run_anim.clone();
                    self.x_v = self.cur_animation.x_v;
                },
                AnimationType::Idle => {
                    self.cur_animation = self.rev_run_anim.clone();
                },
                _ => ()
            }
        }

        if is_key_down(KeyCode::D) {
            match self.cur_animation.anim_type {
                AnimationType::ForwardRun => {
                    // we want to reset the velocity
                    self.x_v = self.cur_animation.x_v;
                },
                AnimationType::ReverseRun => {
                    // we want to reset the velocity
                    self.cur_animation = self.fwd_run_anim.clone();
                    self.x_v = self.cur_animation.x_v;
                    self.x_v = self.x_v * -1.0;
                },
                AnimationType::Idle => {
                    self.cur_animation = self.fwd_run_anim.clone();
                },
                _ => ()
            }
        }

        if is_key_down(KeyCode::Space) {
            self.cur_animation = self.jump_anim.clone();
            self.x_v = self.cur_animation.x_v;
            self.y_v = self.cur_animation.y_v;
        }

        if !is_any_key_down() {
            match self.cur_animation.anim_type {
                AnimationType::ForwardRun => {
                    self.x_v -= self.cur_animation.x_v_decay * dt;

                    if self.x_v <= 0.0 {
                        self.cur_animation = self.idle_anim.clone();
                    }
                },
                AnimationType::ReverseRun => {
                    self.x_v += self.cur_animation.x_v_decay * dt;

                    if self.x_v >= 0.0 {
                        self.cur_animation = self.idle_anim.clone();
                    }
                },
                AnimationType::Jump => { 
                    self.y_v -= self.cur_animation.y_v_decay * dt;

                    if self.x_v >= 0.0 {
                        self.x_v -= self.cur_animation.x_v_decay * dt;
                    }

                    if self.y_v <= 0.0 {
                        self.cur_animation = self.idle_anim.clone();
                    }
                },
                _ => ()
            }
        }

        match self.cur_animation.anim_type {
            AnimationType::Idle => {
                self.color = GREEN;
            },
            AnimationType::ForwardRun | AnimationType::ReverseRun => {
                self.color = ORANGE;
                self.x += self.x_v * dt;
            },
            AnimationType::Jump => {
                self.color = ORANGE;
                self.x += self.x_v * dt;
                self.y -= self.y_v * dt;
            }
        }

        // draw some debugging text with player velocity
        draw_text(&format!("velocity: x: {} | y: {}", self.x_v, self.y_v), 20.0, 10.0, 20.0, DARKGRAY);
        draw_text(&format!("animation: {:?}", self.cur_animation.anim_type), 20.0, 30.0, 20.0, DARKGRAY);

        // we are building a platformer, so there are no Y axis movement (unless jumping)
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }
}

#[macroquad::main("Dangame")]
async fn main() {

    let mut p1 = Player::new(100.0, 500.0, 20.0, 20.0).await;
    let mut entities:Vec<Entity> = Vec::new();

    loop {
        let dt = get_frame_time();

        p1.update(dt, &entities);

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
