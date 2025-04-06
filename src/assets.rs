use std::{cell::RefCell, rc::Rc};

use macroquad::{texture::{load_texture, Texture2D}, time::get_frame_time};

#[derive(Clone, Debug, PartialEq)]
pub enum AnimationType {
    Idle,
    ForwardRun,
    ReverseRun,
    Jump,
    ForwardWalk,
    ReverseWalk,
    Attack1,
    Attack2,
    Attack3,
}

#[derive(Clone, Debug)]
pub struct PlayerAnimation {
    pub anim_type: AnimationType,
    pub texture: Texture2D,
    pub time: f32,

    // Frame is the current frame of the spritesheet
    // NOT the animation frame
    pub sprite_frame: usize,

    // Sequence is a grouping of frames and their FPS
    // This means that the sequence will play 4 frames at 20 FPS, then 1 frame at 10 FPS, then 5 frames at 10 FPS
    pub animation_sequence: Vec<AnimationSequence>,

    // Sequence Index is the current sequence of animations which has 
    // a number of frames and an FPS to play them at.
    pub sequence_index: usize,
    
    // Sequence Frame Index is the current frame of the sequence
    pub sequence_frame_index: usize,

    pub actively_playing: bool,

    // For animations like Idle, we just want to lop them
    pub always_plays: bool,
}

#[derive(Clone, Debug)]
pub struct AnimationSequence {
    frames: usize, 
    fps: f32,
    x_movement: f32,
    y_movement: f32,
}

impl AnimationSequence {
    pub fn new(frames: usize, fps: f32, x_movement: f32, y_movement: f32) -> Self {
        Self {
            frames,
            fps,
            x_movement,
            y_movement,
        }
    }
}

type Delta = (f32, f32);

impl PlayerAnimation {
    pub fn update(&mut self) -> Delta {
        let sequence = &self.animation_sequence[self.sequence_index];
        let (mut dx, mut dy) = (0.0, 0.0);

        if self.actively_playing || self.always_plays {
            self.time += get_frame_time();

            if self.time > 1. / sequence.fps {
                // if we still have sequence frames left to play, then we play them
                if self.sequence_frame_index < sequence.frames - 1 {
                    self.sprite_frame += 1;
                    self.sequence_frame_index += 1;

                    // need to calculate the delta for the next frame
                    dx = sequence.x_movement / sequence.frames as f32;
                    dy = sequence.y_movement / sequence.frames as f32;
                } else {
                    // if we have played all frames in the sequence, then we move to the next sequence
                    if self.sequence_index < self.animation_sequence.len() - 1 {

                        // we need to calculate this BEFORE we move to the next sequence
                        dx = sequence.x_movement / sequence.frames as f32;
                        dy = sequence.y_movement / sequence.frames as f32;

                        self.sprite_frame += 1;
                        self.sequence_index += 1;
                        self.sequence_frame_index = 0;
                    }

                    // if we're done with the animation then we reset
                    if self.sequence_index == self.animation_sequence.len() - 1 {
                        self.reset();
                    }

                    if self.always_plays {
                        self.sequence_index = 0;
                        self.sequence_frame_index = 0;
                        self.sprite_frame = 0;
                    }
                }

                self.time = 0.0;
            }
        }

        (dx, dy)
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.sequence_frame_index = 0;
        self.sequence_index = 0;
        self.sprite_frame = 0;
        self.actively_playing = false;
    }
}

pub struct AnimationBank {
    pub idle_anim: Rc<RefCell<PlayerAnimation>>,
    pub fwd_run_anim: Rc<RefCell<PlayerAnimation>>,
    pub rev_run_anim: Rc<RefCell<PlayerAnimation>>,
    pub jump_anim: Rc<RefCell<PlayerAnimation>>,
    pub fwd_walk_anim: Rc<RefCell<PlayerAnimation>>,
    pub rev_walk_anim: Rc<RefCell<PlayerAnimation>>,
    pub attack_1_anim: Rc<RefCell<PlayerAnimation>>,
    pub attack_2_anim: Rc<RefCell<PlayerAnimation>>,
    pub attack_3_anim: Rc<RefCell<PlayerAnimation>>,
}

impl AnimationBank {
    pub async fn load() -> Self {
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();
        let walk_texture = load_texture("spritesheets/Fighter/Walk.png").await.unwrap();
        let attack_1_texture = load_texture("spritesheets/Fighter/Attack_1.png").await.unwrap();
        let attack_2_texture = load_texture("spritesheets/Fighter/Attack_2.png").await.unwrap();
        let attack_3_texture = load_texture("spritesheets/Fighter/Attack_3.png").await.unwrap();

        let idle_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Idle,
            texture: idle_texture,
            time: 0.0,
            animation_sequence: vec![AnimationSequence::new(6, 20.0, 0.0, 0.0)],
            sprite_frame: 0,
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let fwd_run_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ForwardRun,
            texture: run_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let rev_run_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ReverseRun,
            texture: run_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let jump_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Jump,
            texture: jump_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(2, 3.0, 0.0, 0.0), 
                AnimationSequence::new(3, 20.0, 0.0, 0.0), 
                AnimationSequence::new(2, 3.0, 0.0, 0.0), 
                AnimationSequence::new(3, 20.0, 0.0, 0.0)
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
        }));

        let fwd_walk_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ForwardWalk,
            texture: walk_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let rev_walk_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ReverseWalk,
            texture: walk_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let attack_1_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack1,
            texture: attack_1_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(2, 3.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0), 
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
        }));

        let attack_2_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack2,
            texture: attack_2_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(1, 3.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0), 
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
        }));

        let attack_3_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack3,
            texture: attack_3_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(2, 6.0, 50.0, 0.0), 
                AnimationSequence::new(1, 6.0, 0.0, 0.0), 
                AnimationSequence::new(1, 6.0, 100.0, 0.0), 
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
        }));

        Self {
            idle_anim,
            fwd_run_anim,
            rev_run_anim,
            jump_anim,
            fwd_walk_anim,
            rev_walk_anim,
            attack_1_anim,
            attack_2_anim,
            attack_3_anim,
        }
    }
}
