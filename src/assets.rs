use macroquad::texture::{load_texture, Texture2D};


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnimationType {
    Idle,
    ForwardRun,
    ReverseRun,
    Jump,
    ForwardWalk,
    ReverseWalk,
}

#[derive(Clone, Debug)]
pub struct PlayerAnimation {
    pub anim_type: AnimationType,
    pub sprite_animation: usize,
    pub texture: Texture2D
}

pub struct PlayerSprite {
    pub idle_anim: PlayerAnimation,
    pub fwd_run_anim: PlayerAnimation,
    pub rev_run_anim: PlayerAnimation,
    pub jump_anim: PlayerAnimation,
    pub fwd_walk_anim: PlayerAnimation,
    pub rev_walk_anim: PlayerAnimation,
}

impl PlayerSprite {
    pub async fn load() -> Self {
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();
        let walk_texture = load_texture("spritesheets/Fighter/Walk.png").await.unwrap();

        let idle_anim = PlayerAnimation {
            anim_type: AnimationType::Idle,
            sprite_animation: 0,
            texture: idle_texture,
        };

        let fwd_run_anim = PlayerAnimation {
            anim_type: AnimationType::ForwardRun,
            sprite_animation: 1,
            texture: run_texture.clone(),
        };

        let rev_run_anim = PlayerAnimation {
            anim_type: AnimationType::ReverseRun,
            sprite_animation: 1,
            texture: run_texture,
        };

        let jump_anim = PlayerAnimation {
            anim_type: AnimationType::Jump,
            sprite_animation: 2,
            texture: jump_texture,
        };

        let fwd_walk_anim = PlayerAnimation {
            anim_type: AnimationType::ForwardWalk,
            sprite_animation: 3,
            texture: walk_texture.clone(),
        };

        let rev_walk_anim = PlayerAnimation {
            anim_type: AnimationType::ReverseWalk,
            sprite_animation: 3,
            texture: walk_texture,
        };

        Self {
            idle_anim,
            fwd_run_anim,
            rev_run_anim,
            jump_anim,
            fwd_walk_anim,
            rev_walk_anim,
        }
    }
}
