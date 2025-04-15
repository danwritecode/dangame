use macroquad::{
    input::{KeyCode, is_key_down},
    math::{Vec2, vec2},
    texture::Texture2D,
};
use macroquad_platformer::{Actor, World};
use std::{cell::RefCell, rc::Rc};

use common::constants::*;
use common::animation_deltas::UpdateDeltas;

use common::animation::{
        AnimationSequence, AnimationType, CharacterTextures, CharacterType, PlayerAnimationState, Facing
};
use super::character::CharacterTrait;

pub struct Character2 {
    x_v: f32,
    y_v: f32,
    facing: Facing,
    animations: Character2Animations,
    state: Rc<RefCell<PlayerAnimationState>>,
    actor: Actor,
    world: Rc<RefCell<World>>,
    client_id: Option<u64>
}

impl CharacterTrait for Character2 {
    fn update(&mut self, dt: f32) {
        self.update_physics(dt);
        self.update_animation();
    }

    fn set_client_id(&mut self) {
        self.client_id = Some(0);
    }

    fn get_client_id(&self) -> Option<u64> {
        self.client_id
    }

    fn get_anim_type(&self) -> AnimationType {
        self.state.borrow().anim_type.clone()
    }

    fn get_character_type(&self) -> CharacterType {
        self.state.borrow().character_type.clone()
    }

    fn get_position(&self) -> Vec2 {
        let player_pos = self.world.borrow_mut().actor_pos(self.actor);
        Vec2::new(player_pos.x, player_pos.y)
    }

    fn get_size(&self) -> (i32, i32) {
        let player_size = self.world.borrow_mut().actor_size(self.actor);
        (player_size.0, player_size.1)
    }

    fn get_actor(&self) -> Actor {
        self.actor
    }

    fn get_texture(&self, textures: &Rc<CharacterTextures>) -> Rc<Texture2D> {
        let texture = textures.get_texture(
            &self.state.borrow().character_type,
            &self.state.borrow().anim_type,
        );
        Rc::clone(&texture)
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

impl Character2 {
    pub async fn new(x: f32, y: f32, width: i32, height: i32, world: Rc<RefCell<World>>, client_id: Option<u64>) -> Self {
        let animation_bank = Character2Animations::load().await;
        let state = animation_bank.idle_anim.clone();
        let collider = world
            .borrow_mut()
            .add_actor(vec2(x, y), width as i32, height as i32);

        Self {
            x_v: 0.0,
            y_v: 0.0,
            facing: Facing::Right,
            state,
            animations: animation_bank,
            actor: collider,
            world,
            client_id,
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
                    self.state = Rc::clone(&self.animations.idle_anim);
                }
                AnimationType::Crouch => {
                    self.state = Rc::clone(&self.animations.crouch_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::ForwardRun => {
                    self.state = Rc::clone(&self.animations.fwd_run_anim);
                }
                AnimationType::ReverseRun => {
                    self.state = Rc::clone(&self.animations.rev_run_anim);
                }
                AnimationType::ForwardWalk => {
                    self.state = Rc::clone(&self.animations.fwd_walk_anim);
                }
                AnimationType::ReverseWalk => {
                    self.state = Rc::clone(&self.animations.rev_walk_anim);
                }
                AnimationType::Jump => {
                    self.state = Rc::clone(&self.animations.jump_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::JumpMoving => {
                    self.state = Rc::clone(&self.animations.jump_anim_moving);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Landing => {
                    self.state = Rc::clone(&self.animations.landing_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Attack1 => {
                    self.state = Rc::clone(&self.animations.attack_1_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Attack2 => {
                    self.state = Rc::clone(&self.animations.attack_2_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::Attack3 => {
                    self.state = Rc::clone(&self.animations.attack_3_anim);
                    self.state.borrow_mut().actively_playing = true;
                }
                AnimationType::SoaringKick => {
                    self.state = Rc::clone(&self.animations.soaring_kick_anim);
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

pub struct Character2Animations {
    pub idle_anim: Rc<RefCell<PlayerAnimationState>>,
    pub crouch_anim: Rc<RefCell<PlayerAnimationState>>,
    pub fwd_run_anim: Rc<RefCell<PlayerAnimationState>>,
    pub rev_run_anim: Rc<RefCell<PlayerAnimationState>>,
    pub jump_anim: Rc<RefCell<PlayerAnimationState>>,
    pub jump_anim_moving: Rc<RefCell<PlayerAnimationState>>,
    pub landing_anim: Rc<RefCell<PlayerAnimationState>>,
    pub fwd_walk_anim: Rc<RefCell<PlayerAnimationState>>,
    pub rev_walk_anim: Rc<RefCell<PlayerAnimationState>>,
    pub attack_1_anim: Rc<RefCell<PlayerAnimationState>>,
    pub attack_2_anim: Rc<RefCell<PlayerAnimationState>>,
    pub attack_3_anim: Rc<RefCell<PlayerAnimationState>>,
    pub soaring_kick_anim: Rc<RefCell<PlayerAnimationState>>,
}

impl Character2Animations {
    pub async fn load() -> Self {
        let idle_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Idle,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            animation_sequence: vec![AnimationSequence::new(6, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sprite_frame: 0,
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let crouch_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Crouch,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            animation_sequence: vec![AnimationSequence::new(1, 20.0, 0.0, 0.0, 0.0, 0.0, 60, 28)],
            sprite_frame: 0,
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let fwd_run_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::ForwardRun,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let rev_run_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::ReverseRun,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let jump_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Jump,
            character_type: CharacterType::Shinobi,
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

        let jump_anim_moving = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::JumpMoving,
            character_type: CharacterType::Shinobi,
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

        let landing_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Landing,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(2, 10.0, 0.0, 0.0, 0.0, 0.0, 70, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: false,
            is_interuptable: true,
        }));

        let fwd_walk_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::ForwardWalk,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let rev_walk_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::ReverseWalk,
            character_type: CharacterType::Shinobi,
            time: 0.0,
            sprite_frame: 0,
            animation_sequence: vec![AnimationSequence::new(8, 20.0, 0.0, 0.0, 0.0, 0.0, 93, 28)],
            sequence_index: 0,
            sequence_frame_index: 0,
            actively_playing: false,
            always_plays: true,
            is_interuptable: true,
        }));

        let attack_1_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Attack1,
            character_type: CharacterType::Shinobi,
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

        let attack_2_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Attack2,
            character_type: CharacterType::Shinobi,
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

        let attack_3_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::Attack3,
            character_type: CharacterType::Shinobi,
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

        let soaring_kick_anim = Rc::new(RefCell::new(PlayerAnimationState {
            anim_type: AnimationType::SoaringKick,
            character_type: CharacterType::Shinobi,
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
