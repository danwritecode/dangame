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
}

#[derive(Clone, Debug)]
pub struct AnimationSequence {
    frames: usize, 
    fps: f32,
    x_movement: f32,
    y_movement: f32,
    x_accleration: f32,
    y_accleration: f32,
}

impl AnimationSequence {
    pub fn new(frames: usize, fps: f32, x_movement: f32, y_movement: f32, x_accleration: f32, y_accleration: f32) -> Self {
        Self {
            frames,
            fps,
            x_movement,
            y_movement,
            x_accleration,
            y_accleration,
        }
    }
}

type PosDelta = (f32, f32);
type VelDelta = (f32, f32);

pub struct UpdateDelta {
    pub pos_delta: PosDelta,
    pub vel_delta: VelDelta,
}

impl Default for UpdateDelta {
    fn default() -> Self {
        Self { pos_delta: (0.0, 0.0), vel_delta: (0.0, 0.0) }
    }
}

impl PlayerAnimation {
    pub fn update(&mut self) -> UpdateDelta {
        let sequence = &self.animation_sequence.get(self.sequence_index); 
        let sequence = match sequence {
            Some(sequence) => sequence,
            None => return UpdateDelta::default(),
        };

        let mut delta = UpdateDelta::default();

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
            animation_sequence: vec![AnimationSequence::new(6, 20.0, 0.0, 0.0, 0.0, 0.0)],
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
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0)],
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
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0)],
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
                AnimationSequence::new(1, 40.0, 0.0, 0.0, 0.0, 10.0), 
                AnimationSequence::new(2, 25.0, 0.0, 0.0, 0.0, 200.0), 
                AnimationSequence::new(1, 10.0, 0.0, 0.0, 0.0, 400.0), 
                AnimationSequence::new(1, 1.0, 0.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(4, 15.0, 0.0, 0.0, 0.0, 0.0), 
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
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0)],
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
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0)],
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
                AnimationSequence::new(2, 3.0, 0.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0), 
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
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0), 
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
                AnimationSequence::new(2, 8.0, 75.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(1, 8.0, 0.0, 0.0, 0.0, 0.0), 
                AnimationSequence::new(1, 8.0, 300.0, 0.0, 0.0, 0.0), 
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
