use std::rc::Rc;
use macroquad::{texture::Texture2D, time::get_frame_time};

use crate::types::UpdateDeltas;

#[derive(Clone, Debug, PartialEq)]
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


pub struct PlayerAnimation {
    pub anim_type: AnimationType,
    pub texture: Rc<Texture2D>,
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

impl PlayerAnimation {
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
