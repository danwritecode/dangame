use macroquad::prelude::{animation::{AnimatedSprite, Animation}, *};

struct Player {
    pos: Vec2,
    speed: f32,
}

#[macroquad::main("Dangame")]
async fn main() {
    let mut player = Player {
        pos: vec2(100., 100.),
        speed: 300.,
    };

    let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
    let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
    let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();

    let mut sprite = AnimatedSprite::new(128, 128,
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
                fps: 20
            },
        ],
        true
    );

    let mut cur_texture = &idle_texture;
    let mut flip_x = false;
    let mut jump_frames = vec![];

    loop {
        let dt = get_frame_time();
        let cur_animation = sprite.current_animation();
        let is_last_frame = sprite.is_last_frame();

        if is_key_down(KeyCode::W) {
            player.pos.y -= player.speed * dt;
        }
        if is_key_down(KeyCode::S) {
            player.pos.y += player.speed * dt;
        }

        if is_key_down(KeyCode::A) {
            player.pos.x -= player.speed * dt;
            sprite.set_animation(1);
            cur_texture = &run_texture;
            flip_x = true;
        }

        if is_key_down(KeyCode::D) {
            player.pos.x += player.speed * dt;
            sprite.set_animation(1);
            cur_texture = &run_texture;
            flip_x = false;
        }

        if is_key_down(KeyCode::Space) {
            sprite.set_animation(2);
            cur_texture = &jump_texture;
            jump_frames = jump_animation(player.pos.x, player.pos.y);
        }

        // if jump animation 
        if cur_animation == 2 {
            if is_last_frame {
                sprite.set_animation(0);            
                cur_texture = &idle_texture;
            } else {
                if jump_frames.len() != 0 {
                    let (x, y) = jump_frames.remove(0);
                    player.pos.x = x;
                    player.pos.y = y;
                    sprite.playing;
                }
            }
        }

        if !is_any_key_down() {
            if cur_animation != 2 {
                sprite.set_animation(0);
                cur_texture = &idle_texture;
            }
        }

        clear_background(BLACK);

        draw_texture_ex(
            cur_texture,
            player.pos.x,
            player.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(sprite.frame().source_rect),
                dest_size: Some(sprite.frame().dest_size),
                flip_x,
                ..Default::default()
            }
        );

        sprite.update();
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

fn jump_animation(start_x: f32, start_y: f32) -> Vec<(f32, f32)> {
    return vec![
        (start_x, start_y),
        (start_x, start_y),
        (start_x, start_y - 20.),
        (start_x, start_y - 40.),
        (start_x, start_y - 60.),
        // this is the top point of the jump
        (start_x, start_y - 60.),
        (start_x, start_y - 40.),
        (start_x, start_y - 20.),
        (start_x, start_y),
        (start_x, start_y),
        (start_x, start_y),
    ];
}
