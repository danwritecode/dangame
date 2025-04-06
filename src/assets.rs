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
}

#[derive(Clone, Debug)]
pub struct PlayerAnimation {
    pub anim_type: AnimationType,
    pub sprite_animation: usize,
    pub texture: Texture2D,
    pub time: f32,

    // Frame is the current frame of the spritesheet
    // NOT the animation frame
    pub sprite_frame: usize,

    // Sequence is a grouping of frames and their FPS
    // Example: (4, 20), (1, 10), (5, 10)
    // This means that the sequence will play 4 frames at 20 FPS, then 1 frame at 10 FPS, then 5 frames at 10 FPS
    pub sequence: Vec<(usize, f32)>,

    // Sequence Index is the current sequence of animations which has 
    // a number of frames and an FPS to play them at.
    pub sequence_index: usize,
    
    // Sequence Frame Index is the current frame of the sequence
    pub sequence_frame_index: usize,

    pub actively_playing: bool,

    // For animations like Idle, we just want to lop them
    pub always_plays: bool,
}

impl PlayerAnimation {
    pub fn as_mut(&mut self) -> &mut PlayerAnimation {
        self
    }
}

impl PlayerAnimation {
    pub fn update(&mut self) {
        let (sequence_frames, fps) = &self.sequence[self.sequence_index];

        if self.actively_playing || self.always_plays {
            self.time += get_frame_time();

            if self.time > 1. / fps {
                // if we still have sequence frames left to play, then we play them
                if self.sequence_frame_index < sequence_frames - 1 {
                    self.sprite_frame += 1;
                    self.sequence_frame_index += 1;
                }

                // if we have played all frames in the sequence, then we move to the next sequence
                if self.sequence_frame_index == sequence_frames - 1 {
                    if self.sequence_index < self.sequence.len() - 1 {
                        self.sprite_frame += 1;
                        self.sequence_index += 1;
                        self.sequence_frame_index = 0;
                    }

                    if self.sequence_index == self.sequence.len() - 1 {
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

        // self.frame %= sequence_frames;
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
}

impl AnimationBank {
    pub async fn load() -> Self {
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump.png").await.unwrap();
        let walk_texture = load_texture("spritesheets/Fighter/Walk.png").await.unwrap();
        let attack_1_texture = load_texture("spritesheets/Fighter/Attack_1.png").await.unwrap();

        let idle_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Idle,
            sprite_animation: 0,
            texture: idle_texture,
            time: 0.0,
            sequence: vec![(6, 20.0)],
            sprite_frame: 0,
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let fwd_run_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ForwardRun,
            sprite_animation: 1,
            texture: run_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            sequence: vec![(8, 20.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let rev_run_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ReverseRun,
            sprite_animation: 1,
            texture: run_texture,
            time: 0.0,
            sprite_frame: 0,
            sequence: vec![(8, 20.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let jump_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Jump,
            sprite_animation: 2,
            texture: jump_texture,
            time: 0.0,
            sprite_frame: 0,
            sequence: vec![(2, 3.0), (3, 20.0), (2, 3.0), (3, 20.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
        }));

        let fwd_walk_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ForwardWalk,
            sprite_animation: 3,
            texture: walk_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            sequence: vec![(8, 20.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let rev_walk_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ReverseWalk,
            sprite_animation: 3,
            texture: walk_texture,
            time: 0.0,
            sprite_frame: 0,
            sequence: vec![(8, 20.0)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
        }));

        let attack_1_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack1,
            sprite_animation: 3,
            texture: attack_1_texture,
            time: 0.0,
            sprite_frame: 0,
            sequence: vec![(2, 3.0), (1, 3.0), (1, 3.0)],
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
        }
    }
}
