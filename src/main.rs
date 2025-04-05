use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};

use assets::{AnimationType, PlayerAnimation, PlayerSprite};

mod assets;

const RUN_SPEED: f32 = 300.0;
const WALK_SPEED: f32 = 150.0;
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

struct Player<'a> {
    x: f32,
    y: f32,
    x_v: f32,
    y_v: f32,
    facing: Facing,
    width: f32,
    height: f32,
    state: &'a PlayerAnimation,
    assets: &'a PlayerSprite,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Facing {
    Left,
    Right,
}

impl<'a> Player<'a> {
    async fn new(x: f32, y: f32, width: f32, height: f32, assets: &'a PlayerSprite) -> Self {
        Self {
            x,
            y,
            x_v: 0.0,
            y_v: 0.0,
            width,
            height,
            facing: Facing::Right,
            state: &assets.idle_anim,
            assets
        }
    }

    fn update(&mut self, dt: f32, entities: &Vec<Entity>) {
        let wants_walk_left = is_key_down(KeyCode::A);
        let wants_walk_right = is_key_down(KeyCode::D);

        let wants_run_left = is_key_down(KeyCode::A) && is_key_down(KeyCode::LeftShift);
        let wants_run_right = is_key_down(KeyCode::D) && is_key_down(KeyCode::LeftShift);

        let wants_jump = is_key_down(KeyCode::Space);
        let wants_nothing = !is_any_key_down();

        let mut next_animation_state = &self.state.anim_type;
        let is_airborn = self.y < FLOOR - self.height;
        let is_grounded = self.y >= FLOOR - self.height;

        // if y is above floor, we are jumping
        if is_airborn {
            self.y_v -= GRAVITY * dt;
            self.y -=  self.y_v * dt;
        }

        if is_grounded { 
            self.y = FLOOR - self.height; 
            self.y_v = 0.0; 
            next_animation_state = &AnimationType::Idle;
        }

        if wants_walk_left {
            self.x_v = WALK_SPEED * -1.0;
            self.facing = Facing::Left;
            if is_grounded { next_animation_state = &AnimationType::ReverseWalk; }
            
        }

        if wants_walk_right {
            self.x_v = WALK_SPEED;
            self.facing = Facing::Right;
            if is_grounded { next_animation_state = &AnimationType::ForwardWalk; }
        }

        if wants_run_left {
            self.x_v = RUN_SPEED * -1.0;
            self.facing = Facing::Left;
            if is_grounded { next_animation_state = &AnimationType::ReverseRun; }
            
        }

        if wants_run_right {
            self.x_v = RUN_SPEED;
            self.facing = Facing::Right;
            if is_grounded { next_animation_state = &AnimationType::ForwardRun; }
        }

        if wants_jump {
            if is_grounded {
                self.y_v = JUMP_SPEED;
                next_animation_state = &AnimationType::Jump;
            }
        }

        if wants_nothing {
            if is_grounded {
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
                AnimationType::ForwardWalk => {
                    self.state = &self.assets.fwd_walk_anim;
                },
                AnimationType::ReverseWalk => {
                    self.state = &self.assets.rev_walk_anim;
                }
                AnimationType::Jump => {
                    self.state = &self.assets.jump_anim;
                },
            }
        }
    }
}

#[macroquad::main("Dangame")]
async fn main() {
    let mut player_sprite = PlayerSprite::load().await;

    let mut p1 = Player::new(100.0, FLOOR - 20.0, 20.0, 80.0, &mut player_sprite).await;
    let mut entities:Vec<Entity> = Vec::new();

    let mut sprites = AnimatedSprite::new(128, 128,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 6,
                fps: 20
            },
            Animation {
                name: "run".to_string(),
                row: 0,
                frames: 8,
                fps: 20
            },
            Animation {
                name: "jump".to_string(),
                row: 0,
                frames: 10,
                fps: 4
            },
            Animation {
                name: "walk".to_string(),
                row: 0,
                frames: 8,
                fps: 20
            },
        ],
        true
    );


    loop {
        let dt = get_frame_time();

        p1.update(dt, &entities);

        sprites.set_animation(p1.state.sprite_animation);
        draw_texture_ex(
            &p1.state.texture,
            p1.x - 50.0,
            p1.y - 50.0,
            WHITE,
            DrawTextureParams {
                source: Some(sprites.frame().source_rect),
                dest_size: Some(sprites.frame().dest_size),
                flip_x: p1.facing == Facing::Left,
                ..Default::default()
            }
        );

        draw_line(0.0, FLOOR, screen_width(), FLOOR, 2.0, RED);

        // draw some debugging text with player velocity
        draw_text(&format!("vx: {} | vy: {}", p1.x_v, p1.y_v), 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(&format!("x: {} | y: {}", p1.x, p1.y), 20.0, 35.0, 20.0, DARKGRAY);
        draw_text(&format!("animation: {:?}", p1.state.anim_type), 20.0, 50.0, 20.0, DARKGRAY);
        draw_text(&format!("{:?}", FLOOR - p1.height), 20.0, 70.0, 20.0, DARKGRAY);

        sprites.update();
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
