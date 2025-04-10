use std::{cell::RefCell, rc::Rc};

use macroquad::{
    input::{is_key_down, KeyCode},
    math::{vec2, Vec2},
    texture::{load_texture, Texture2D},
    time::get_frame_time,
};
use macroquad_platformer::{Actor, World};

use crate::constants::*;
use crate::types::update_delta::UpdateDeltas;

use super::characters::{CharacterTrait, Facing};

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

pub struct Character1 {
    x_v: f32,
    y_v: f32,
    facing: Facing,
    animation_bank: Character1AnimationBank,
    state: Rc<RefCell<PlayerAnimation>>,
    actor: Actor,
    world: Rc<RefCell<World>>,
}

impl CharacterTrait for Character1 {
    fn update(&mut self, dt: f32) {
        self.update_physics(dt);
        self.update_animation();
    }

    fn get_actor(&self) -> Actor {
        self.actor
    }

    fn get_texture(&self) -> Texture2D {
        self.state.borrow().texture.clone() // TODO: need to fix this
    }

    fn get_facing(&self) -> Facing {
        self.facing.clone()
    }

    fn get_sprite_frame(&self) -> usize {
        self.state.borrow().sprite_frame
    }

    fn get_velocity(&self) -> Vec2 {
        Vec2::new(self.x_v, self.y_v)
    }
}

impl Character1 {
    pub async fn new(x: f32, y: f32, width: i32, height: i32, world: Rc<RefCell<World>>) -> Self {
        let animation_bank = Character1AnimationBank::load().await;
        let state = animation_bank.idle_anim.clone();
        let collider = world
            .borrow_mut()
            .add_actor(vec2(x, y), width as i32, height as i32);

        Self {
            x_v: 0.0,
            y_v: 0.0,
            facing: Facing::Right,
            state,
            animation_bank,
            actor: collider,
            world,
        }
    }

    fn update_physics(&mut self, dt: f32) {
        let wants_crouch = is_key_down(KeyCode::C);

        let wants_walk_left = is_key_down(KeyCode::A);
        let wants_walk_right = is_key_down(KeyCode::D);

        let wants_run_left = is_key_down(KeyCode::A) && is_key_down(KeyCode::LeftShift);
        let wants_run_right = is_key_down(KeyCode::D) && is_key_down(KeyCode::LeftShift);

        let wants_jump = is_key_down(KeyCode::Space);

        let wants_attack_1 = is_key_down(KeyCode::E);
        let wants_attack_2 = is_key_down(KeyCode::Q);
        let wants_kick = is_key_down(KeyCode::R);

        let wants_nothing = !is_any_key_down();

        let pos = self.world.borrow().actor_pos(self.actor);
        let is_grounded = self
            .world
            .borrow()
            .collide_check(self.actor, pos + vec2(0., 1.));
        let is_colliding_right = self
            .world
            .borrow()
            .collide_check(self.actor, pos + vec2(1., 0.));
        let is_colliding_left = self
            .world
            .borrow()
            .collide_check(self.actor, pos - vec2(1., 0.));

        let is_airborn = !is_grounded;
        let is_actively_playing = self.state.borrow().actively_playing;
        let is_interuptable = self.state.borrow().is_interuptable;
        let was_just_airborn = is_grounded && self.y_v < 0.0;

        let mut next_animation_state = self.state.borrow().anim_type.clone();

        if is_airborn {
            self.y_v -= GRAVITY * dt;

            if wants_kick {
                next_animation_state = AnimationType::SoaringKick;
            }
        }

        // this represents when a player WAS jumping but just touched the ground
        // they have negative y velocity because they were falling back down
        if was_just_airborn {
            self.y_v = 0.0;
            next_animation_state = AnimationType::Landing;
        }

        // if grounded the we have friction and reset to 0.0
        if is_grounded {
            self.x_v = 0.0;
        }

        // if we are moving left and hit something on our left, we reset to 0.0
        if is_colliding_left && self.x_v < 0.0 {
            self.x_v = 0.0;
        }
        // if we are moving right and hit something on our right, we reset to 0.0
        if is_colliding_right && self.x_v > 0.0 {
            self.x_v = 0.0;
        }

        if wants_nothing {
            if is_grounded && !was_just_airborn {
                next_animation_state = AnimationType::Idle;
            }
        }

        if is_interuptable {
            if wants_walk_left {
                if !wants_crouch {
                    self.x_v = WALK_SPEED * -1.0;
                    self.facing = Facing::Left;
                    if is_grounded {
                        next_animation_state = AnimationType::ReverseWalk;
                    }
                }
            }

            if wants_walk_right {
                if !wants_crouch {
                    self.x_v = WALK_SPEED;
                    self.facing = Facing::Right;
                    if is_grounded {
                        next_animation_state = AnimationType::ForwardWalk;
                    }
                }
            }

            if wants_run_left {
                if !wants_crouch {
                    self.x_v = RUN_SPEED * -1.0;
                    self.facing = Facing::Left;
                    if is_grounded {
                        next_animation_state = AnimationType::ReverseRun;
                    }
                }
            }

            if wants_run_right {
                if !wants_crouch {
                    self.x_v = RUN_SPEED;
                    self.facing = Facing::Right;
                    if is_grounded {
                        next_animation_state = AnimationType::ForwardRun;
                    }
                }
            }

            if wants_jump {
                if is_grounded && !wants_crouch {
                    if self.x_v == 0.0 {
                        next_animation_state = AnimationType::Jump;
                    } else {
                        next_animation_state = AnimationType::JumpMoving;
                    }
                }
            }

            if wants_attack_1 {
                if is_grounded {
                    next_animation_state = AnimationType::Attack1;
                }
            }

            if wants_attack_2 {
                if is_grounded {
                    next_animation_state = AnimationType::Attack2;
                }
            }

            if wants_kick {
                if is_grounded {
                    next_animation_state = AnimationType::Attack3;
                }
            }

            if wants_crouch {
                next_animation_state = AnimationType::Crouch;
            }
        }

        self.world.borrow_mut().move_h(self.actor, self.x_v * dt);
        self.world
            .borrow_mut()
            .move_v(self.actor, (self.y_v * -1.0) * dt);

        if next_animation_state != self.state.borrow().anim_type && !is_actively_playing {
            // we decided above if we want to change animations or not
            // if we want to change animations, we need to stop the current animation
            self.state.borrow_mut().reset();

            match next_animation_state {
                AnimationType::Idle => {
                    self.state = Rc::clone(&self.animation_bank.idle_anim);
                }
                AnimationType::Crouch => {
                    self.state = Rc::clone(&self.animation_bank.crouch_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::ForwardRun => {
                    self.state = Rc::clone(&self.animation_bank.fwd_run_anim);
                }
                AnimationType::ReverseRun => {
                    self.state = Rc::clone(&self.animation_bank.rev_run_anim);
                }
                AnimationType::ForwardWalk => {
                    self.state = Rc::clone(&self.animation_bank.fwd_walk_anim);
                }
                AnimationType::ReverseWalk => {
                    self.state = Rc::clone(&self.animation_bank.rev_walk_anim);
                }
                AnimationType::Jump => {
                    self.state = Rc::clone(&self.animation_bank.jump_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::JumpMoving => {
                    self.state = Rc::clone(&self.animation_bank.jump_anim_moving);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Landing => {
                    self.state = Rc::clone(&self.animation_bank.landing_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Attack1 => {
                    self.state = Rc::clone(&self.animation_bank.attack_1_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Attack2 => {
                    self.state = Rc::clone(&self.animation_bank.attack_2_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Attack3 => {
                    self.state = Rc::clone(&self.animation_bank.attack_3_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::SoaringKick => {
                    self.state = Rc::clone(&self.animation_bank.soaring_kick_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
            }
        }
    }

    fn update_animation(&mut self) {
        let deltas = self.state.borrow_mut().update();
        self.apply_animation_deltas(&deltas);
    }

    fn apply_animation_deltas(&mut self, delta: &UpdateDeltas) {
        let world = Rc::clone(&self.world);

        if self.facing == Facing::Left {
            world
                .borrow_mut()
                .move_h(self.actor, delta.pos_delta.0 * -1.0);
            if delta.vel_delta.0 != 0.0 {
                self.x_v -= delta.vel_delta.0;
            }
        } else {
            world.borrow_mut().move_h(self.actor, delta.pos_delta.0);
            if delta.vel_delta.0 != 0.0 {
                self.x_v += delta.vel_delta.0;
            }
        }

        world.borrow_mut().move_v(self.actor, delta.pos_delta.1);
        if delta.vel_delta.1 != 0.0 {
            self.y_v += delta.vel_delta.1;
        }

        // set actor size
        world
            .borrow_mut()
            .set_actor_size(self.actor, delta.width, delta.height);
    }
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

    /// If an animation is interruptable, it means that it can be interrupted by another animation
    pub is_interuptable: bool,
}

#[derive(Clone, Debug)]
pub struct AnimationSequence {
    frames: usize,
    fps: f32,
    x_movement: f32,
    y_movement: f32,
    x_accleration: f32,
    y_accleration: f32,

    /// the difference in height during an animation
    height: i32,
    /// the difference in width during an animation
    width: i32,
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

pub struct Character1AnimationBank {
    pub idle_anim: Rc<RefCell<PlayerAnimation>>,
    pub crouch_anim: Rc<RefCell<PlayerAnimation>>,
    pub fwd_run_anim: Rc<RefCell<PlayerAnimation>>,
    pub rev_run_anim: Rc<RefCell<PlayerAnimation>>,
    pub jump_anim: Rc<RefCell<PlayerAnimation>>,
    pub jump_anim_moving: Rc<RefCell<PlayerAnimation>>,
    pub landing_anim: Rc<RefCell<PlayerAnimation>>,
    pub fwd_walk_anim: Rc<RefCell<PlayerAnimation>>,
    pub rev_walk_anim: Rc<RefCell<PlayerAnimation>>,
    pub attack_1_anim: Rc<RefCell<PlayerAnimation>>,
    pub attack_2_anim: Rc<RefCell<PlayerAnimation>>,
    pub attack_3_anim: Rc<RefCell<PlayerAnimation>>,
    pub soaring_kick_anim: Rc<RefCell<PlayerAnimation>>,
}

impl Character1AnimationBank {
    pub async fn load() -> Self {
        let idle_texture = load_texture("spritesheets/Fighter/Idle.png").await.unwrap();
        let crouch_texture = load_texture("spritesheets/Fighter/Crouch.png")
            .await
            .unwrap();
        let run_texture = load_texture("spritesheets/Fighter/Run.png").await.unwrap();
        let jump_texture = load_texture("spritesheets/Fighter/Jump_02.png")
            .await
            .unwrap();
        let walk_texture = load_texture("spritesheets/Fighter/Walk.png").await.unwrap();
        let landing_texture = load_texture("spritesheets/Fighter/Landing.png")
            .await
            .unwrap();
        let attack_1_texture = load_texture("spritesheets/Fighter/Attack_1.png")
            .await
            .unwrap();
        let attack_2_texture = load_texture("spritesheets/Fighter/Attack_2.png")
            .await
            .unwrap();
        let attack_3_texture = load_texture("spritesheets/Fighter/Attack_3.png")
            .await
            .unwrap();

        let idle_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Idle,
            texture: idle_texture,
            time: 0.0,
            animation_sequence: vec![AnimationSequence::new(6, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sprite_frame: 0,
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let crouch_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Crouch,
            texture: crouch_texture,
            time: 0.0,
            animation_sequence: vec![AnimationSequence::new(1, 20.0, 0.0, 0.0, 0.0, 0.0, 60, 28)],
            sprite_frame: 0,
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let fwd_run_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ForwardRun,
            texture: run_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let rev_run_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ReverseRun,
            texture: run_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let jump_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Jump,
            texture: jump_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(1, 20.0, 0.0, 0.0, 0.0, 500.0, 93, 28),
                AnimationSequence::new(2, 20.0, 0.0, 0.0, 0.0, 0.0, 70, 28),
                AnimationSequence::new(4, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let jump_anim_moving = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::JumpMoving,
            texture: jump_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(1, 20.0, 0.0, 0.0, 0.0, 500.0, 93, 28),
                AnimationSequence::new(2, 20.0, 0.0, 0.0, 0.0, 0.0, 70, 28),
                AnimationSequence::new(5, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let landing_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Landing,
            texture: landing_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(2, 10.0, 0.0, 0.0, 0.0, 0.0, 70, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let fwd_walk_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ForwardWalk,
            texture: walk_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let rev_walk_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::ReverseWalk,
            texture: walk_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let attack_1_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack1,
            texture: attack_1_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(2, 3.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let attack_2_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack2,
            texture: attack_2_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
                AnimationSequence::new(1, 3.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let attack_3_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::Attack3,
            texture: attack_3_texture.clone(),
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(2, 8.0, 75.0, 0.0, 0.0, 0.0, 93, 28),
                AnimationSequence::new(1, 8.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
                AnimationSequence::new(1, 8.0, 50.0, 0.0, 0.0, 0.0, 93, 28),
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let soaring_kick_anim = Rc::new(RefCell::new(PlayerAnimation {
            anim_type: AnimationType::SoaringKick,
            texture: attack_3_texture,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![
                AnimationSequence::new(2, 8.0, 0.0, 0.0, 1250.0, -200.0, 93, 28),
                AnimationSequence::new(2, 8.0, 0.0, 0.0, 0.0, 0.0, 93, 28),
            ],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: false,
        }));

        Self {
            idle_anim,
            crouch_anim,
            fwd_run_anim,
            rev_run_anim,
            jump_anim,
            jump_anim_moving,
            landing_anim,
            fwd_walk_anim,
            rev_walk_anim,
            attack_1_anim,
            attack_2_anim,
            attack_3_anim,
            soaring_kick_anim,
        }
    }
}

fn is_any_key_down() -> bool {
    if is_key_down(KeyCode::W) {
        return true;
    }
    if is_key_down(KeyCode::S) {
        return true;
    }
    if is_key_down(KeyCode::A) {
        return true;
    }
    if is_key_down(KeyCode::D) {
        return true;
    }
    if is_key_down(KeyCode::Space) {
        return true;
    }
    false
}
