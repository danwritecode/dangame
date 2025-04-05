use macroquad::texture::load_texture;

use crate::{AnimationType, PlayerAnimation};


pub struct Assets {
    pub idle_anim: PlayerAnimation,
    pub fwd_run_anim: PlayerAnimation,
    pub rev_run_anim: PlayerAnimation,
    pub jump_anim: PlayerAnimation,
}

impl Assets {
    pub async fn load() -> Self {
        
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();

        let idle_anim = PlayerAnimation {
            anim_type: AnimationType::Idle,
            sprite_frames: 6,
            anim_frames: 6,
            fps: 20,
            texture: idle_texture,
        };

        let fwd_run_anim = PlayerAnimation {
            anim_type: AnimationType::ForwardRun,
            sprite_frames: 8,
            anim_frames: 8,
            fps: 20,
            texture: run_texture.clone(),
        };

        let rev_run_anim = PlayerAnimation {
            anim_type: AnimationType::ReverseRun,
            sprite_frames: 8,
            anim_frames: 8,
            fps: 20,
            texture: run_texture,
        };

        let jump_anim = PlayerAnimation {
            anim_type: AnimationType::Jump,
            sprite_frames: 10,
            anim_frames: 10,
            fps: 20,
            texture: jump_texture,
        };

        Self {
            idle_anim,
            fwd_run_anim,
            rev_run_anim,
            jump_anim,
        }
    }
}
