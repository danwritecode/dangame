use macroquad::{math::Vec2, texture::Texture2D};
use macroquad_platformer::Actor;


pub trait CharacterTrait {
    fn update(&mut self, dt: f32);
    fn get_actor(&self) -> Actor;
    fn get_texture(&self) -> Texture2D;
    fn get_facing(&self) -> Facing;
    fn get_sprite_frame(&self) -> usize;
    fn get_velocity(&self) -> Vec2;
}

pub enum CharacterType {
    Character1,
    Character2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}
