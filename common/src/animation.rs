use std::rc::Rc;
use bincode::{Decode, Encode};

use macroquad::time::get_frame_time;
use macroquad::texture::{load_texture, Texture2D};

use crate::animation_deltas::UpdateDeltas;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum AnimationType {
    Idle,
    Crouch,
    ForwardRun,
    ReverseRun,
    Jump,
    JumpMoving,
    Landing,
    ForwardWalk,
    ReverseWalk,
    Attack1,
    Attack2,
    Attack3,
    SoaringKick,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum CharacterType {
    Fighter,
    Shinobi,
    Samurai
}

pub struct PlayerAnimationState {
    pub anim_type: AnimationType,
    pub character_type: CharacterType,
    pub time: f32,

    /// Frame is the current frame of the spritesheetNOT the animation frame
    pub sprite_frame: usize,

    // Sequence is a grouping of frames and their FPS This means that the sequence will play 4 frames at 20 FPS, then 1 frame at 10 FPS, then 5 frames at 10 FPS
    pub animation_sequence: Vec<AnimationSequence>,

    /// Sequence Index is the current sequence of animations which has a number of frames and an FPS to play them at.
    pub sequence_index: usize,

    /// Sequence Frame Index is the current frame of the sequence
    pub sequence_frame_index: usize,

    pub actively_playing: bool,

    /// For animations like Idle, we just want to lop them
    pub always_plays: bool,

    /// If an animation is interruptable, it means that it can be interrupted by another animation
    pub is_interuptable: bool,
}

impl PlayerAnimationState {
    pub fn update(&mut self) -> UpdateDeltas {
        let sequence = &self.animation_sequence.get(self.sequence_index);
        let sequence = match sequence {
            Some(sequence) => sequence,
            None => return UpdateDeltas::default(),
        };

        let mut delta = UpdateDeltas::default();
        delta.height = sequence.height;
        delta.width = sequence.width;

        if self.actively_playing || self.always_plays {
            self.time += get_frame_time();

            if self.time > 1. / sequence.fps {
                // need to process our movement deltas no matter what
                delta.pos_delta.0 = sequence.x_movement / sequence.frames as f32;
                delta.pos_delta.1 = sequence.y_movement / sequence.frames as f32;
                delta.vel_delta.0 = sequence.x_accleration / sequence.frames as f32;
                delta.vel_delta.1 = sequence.y_accleration / sequence.frames as f32;

                let is_last_sequence = self.sequence_index == self.animation_sequence.len() - 1;
                let is_last_sequence_frame = self.sequence_frame_index == sequence.frames - 1;

                if !is_last_sequence_frame {
                    self.sprite_frame += 1;
                    self.sequence_frame_index += 1;
                }

                if is_last_sequence_frame && !is_last_sequence {
                    self.sprite_frame += 1;
                    self.sequence_frame_index = 0;
                    self.sequence_index += 1;
                }

                if is_last_sequence_frame && is_last_sequence {
                    if self.always_plays {
                        self.sequence_index = 0;
                        self.sequence_frame_index = 0;
                        self.sprite_frame = 0;
                    } else {
                        self.actively_playing = false;
                    }
                }

                self.time = 0.0;
            }
        }

        delta
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.sequence_frame_index = 0;
        self.sequence_index = 0;
        self.sprite_frame = 0;
        self.actively_playing = false;
    }
}

#[derive(Clone, Debug)]
pub struct AnimationSequence {
    pub frames: usize,
    pub fps: f32,
    pub x_movement: f32,
    pub y_movement: f32,
    pub x_accleration: f32,
    pub y_accleration: f32,

    /// the difference in height during an animation
    pub height: i32,
    /// the difference in width during an animation
    pub width: i32,
}

impl AnimationSequence {
    pub fn new(
        frames: usize,
        fps: f32,
        x_movement: f32,
        y_movement: f32,
        x_accleration: f32,
        y_accleration: f32,
        height: i32,
        width: i32,
    ) -> Self {
        Self {
            frames,
            fps,
            x_movement,
            y_movement,
            x_accleration,
            y_accleration,
            height,
            width,
        }
    }
}


// Structs to hold all textures for each character
pub struct FighterTextures {
    pub idle: Rc<Texture2D>,
    pub crouch: Rc<Texture2D>,
    pub run: Rc<Texture2D>,
    pub jump: Rc<Texture2D>,
    pub walk: Rc<Texture2D>,
    pub landing: Rc<Texture2D>,
    pub attack1: Rc<Texture2D>,
    pub attack2: Rc<Texture2D>,
    pub attack3: Rc<Texture2D>,
}

pub struct ShinobiTextures {
    pub idle: Rc<Texture2D>,
    pub crouch: Rc<Texture2D>,
    pub run: Rc<Texture2D>,
    pub jump: Rc<Texture2D>,
    pub walk: Rc<Texture2D>,
    pub landing: Rc<Texture2D>,
    pub attack1: Rc<Texture2D>,
    pub attack2: Rc<Texture2D>,
    pub attack3: Rc<Texture2D>,
}

pub struct SamuraiTextures {
    pub idle: Rc<Texture2D>,
    pub crouch: Rc<Texture2D>,
    pub run: Rc<Texture2D>,
    pub jump: Rc<Texture2D>,
    pub walk: Rc<Texture2D>,
    pub landing: Rc<Texture2D>,
    pub attack1: Rc<Texture2D>,
    pub attack2: Rc<Texture2D>,
    pub attack3: Rc<Texture2D>,
}

// Container for all character textures
pub struct CharacterTextures {
    pub fighter: FighterTextures,
    pub shinobi: ShinobiTextures,
    pub samurai: SamuraiTextures,
}

impl FighterTextures {
    pub async fn load() -> Self {
        Self {
            idle: Rc::new(load_texture("assets/spritesheets/Fighter/Idle.png").await.unwrap()),
            crouch: Rc::new(load_texture("assets/spritesheets/Fighter/Crouch.png").await.unwrap()),
            run: Rc::new(load_texture("assets/spritesheets/Fighter/Run.png").await.unwrap()),
            jump: Rc::new(load_texture("assets/spritesheets/Fighter/Jump_02.png").await.unwrap()),
            walk: Rc::new(load_texture("assets/spritesheets/Fighter/Walk.png").await.unwrap()),
            landing: Rc::new(load_texture("assets/spritesheets/Fighter/Landing.png").await.unwrap()),
            attack1: Rc::new(load_texture("assets/spritesheets/Fighter/Attack_1.png").await.unwrap()),
            attack2: Rc::new(load_texture("assets/spritesheets/Fighter/Attack_2.png").await.unwrap()),
            attack3: Rc::new(load_texture("assets/spritesheets/Fighter/Attack_3.png").await.unwrap()),
        }
    }

    pub fn get_texture(&self, animation: &AnimationType) -> Rc<Texture2D> {
        match animation {
            AnimationType::Idle => self.idle.clone(),
            AnimationType::Crouch => self.crouch.clone(),
            AnimationType::ForwardRun | AnimationType::ReverseRun => self.run.clone(),
            AnimationType::Jump | AnimationType::JumpMoving => self.jump.clone(),
            AnimationType::Landing => self.landing.clone(),
            AnimationType::ForwardWalk | AnimationType::ReverseWalk => self.walk.clone(),
            AnimationType::Attack1 => self.attack1.clone(),
            AnimationType::Attack2 => self.attack2.clone(),
            AnimationType::Attack3 | AnimationType::SoaringKick => self.attack3.clone(),
        }
    }
}

impl ShinobiTextures {
    pub async fn load() -> Self {
        Self {
            idle: Rc::new(load_texture("assets/spritesheets/Shinobi/Idle.png").await.unwrap()),
            crouch: Rc::new(load_texture("assets/spritesheets/Shinobi/Idle.png").await.unwrap()),
            run: Rc::new(load_texture("assets/spritesheets/Shinobi/Run.png").await.unwrap()),
            jump: Rc::new(load_texture("assets/spritesheets/Shinobi/Jump.png").await.unwrap()),
            walk: Rc::new(load_texture("assets/spritesheets/Shinobi/Walk.png").await.unwrap()),
            landing: Rc::new(load_texture("assets/spritesheets/Shinobi/Idle.png").await.unwrap()),
            attack1: Rc::new(load_texture("assets/spritesheets/Shinobi/Attack_1.png").await.unwrap()),
            attack2: Rc::new(load_texture("assets/spritesheets/Shinobi/Attack_2.png").await.unwrap()),
            attack3: Rc::new(load_texture("assets/spritesheets/Shinobi/Attack_3.png").await.unwrap()),
        }
    }

    pub fn get_texture(&self, animation: &AnimationType) -> Rc<Texture2D> {
        match animation {
            AnimationType::Idle => self.idle.clone(),
            AnimationType::Crouch => self.crouch.clone(),
            AnimationType::ForwardRun | AnimationType::ReverseRun => self.run.clone(),
            AnimationType::Jump | AnimationType::JumpMoving => self.jump.clone(),
            AnimationType::Landing => self.landing.clone(),
            AnimationType::ForwardWalk | AnimationType::ReverseWalk => self.walk.clone(),
            AnimationType::Attack1 => self.attack1.clone(),
            AnimationType::Attack2 => self.attack2.clone(),
            AnimationType::Attack3 | AnimationType::SoaringKick => self.attack3.clone(),
        }
    }
}

impl SamuraiTextures {
    pub async fn load() -> Self {
        Self {
            idle: Rc::new(load_texture("assets/spritesheets/Samurai/Idle.png").await.unwrap()),
            crouch: Rc::new(load_texture("assets/spritesheets/Samurai/Idle.png").await.unwrap()),
            run: Rc::new(load_texture("assets/spritesheets/Samurai/Run.png").await.unwrap()),
            jump: Rc::new(load_texture("assets/spritesheets/Samurai/Jump.png").await.unwrap()),
            walk: Rc::new(load_texture("assets/spritesheets/Samurai/Walk.png").await.unwrap()),
            landing: Rc::new(load_texture("assets/spritesheets/Samurai/Idle.png").await.unwrap()),
            attack1: Rc::new(load_texture("assets/spritesheets/Samurai/Attack_1.png").await.unwrap()),
            attack2: Rc::new(load_texture("assets/spritesheets/Samurai/Attack_2.png").await.unwrap()),
            attack3: Rc::new(load_texture("assets/spritesheets/Samurai/Attack_3.png").await.unwrap()),
        }
    }

    pub fn get_texture(&self, animation: &AnimationType) -> Rc<Texture2D> {
        match animation {
            AnimationType::Idle => self.idle.clone(),
            AnimationType::Crouch => self.crouch.clone(),
            AnimationType::ForwardRun | AnimationType::ReverseRun => self.run.clone(),
            AnimationType::Jump | AnimationType::JumpMoving => self.jump.clone(),
            AnimationType::Landing => self.landing.clone(),
            AnimationType::ForwardWalk | AnimationType::ReverseWalk => self.walk.clone(),
            AnimationType::Attack1 => self.attack1.clone(),
            AnimationType::Attack2 => self.attack2.clone(),
            AnimationType::Attack3 | AnimationType::SoaringKick => self.attack3.clone(),
        }
    }
}

impl CharacterTextures {
    pub async fn load_all() -> Self {
        let fighter = FighterTextures::load().await;
        let shinobi = ShinobiTextures::load().await;
        let samurai = SamuraiTextures::load().await;
        Self { fighter, shinobi, samurai }
    }

    pub fn get_texture(
        &self,
        character: &CharacterType,
        animation: &AnimationType,
    ) -> Rc<Texture2D> {
        match character {
            CharacterType::Fighter => self.fighter.get_texture(animation),
            CharacterType::Shinobi => self.shinobi.get_texture(animation),
            CharacterType::Samurai => self.samurai.get_texture(animation),
        }
    }
}
