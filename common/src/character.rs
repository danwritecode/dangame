use std::{cell::RefCell, rc::Rc};

use macroquad::{math::Vec2, texture::Texture2D};
use macroquad_platformer::Actor;

use crate::animation::PlayerAnimation;


pub trait CharacterTrait {
    fn update(&mut self, dt: f32);
    fn get_actor(&self) -> Actor;
    fn get_texture(&self) -> Rc<Texture2D>;
    fn get_facing(&self) -> Facing;
    fn get_sprite_frame(&self) -> usize;
    fn get_velocity(&self) -> Vec2;
    fn get_generic_state(&self) -> GenericCharacterState;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

pub struct GenericCharacterState {
    pub x_v: f32,
    pub y_v: f32,
    pub facing: Facing,
    pub state: Rc<RefCell<PlayerAnimation>>
}
