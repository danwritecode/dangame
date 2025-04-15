use std::rc::Rc;

use macroquad::{math::Vec2, texture::Texture2D};
use macroquad_platformer::Actor;

use common::animation::{AnimationType, CharacterTextures, CharacterType, Facing};

#[allow(dead_code)]
pub trait CharacterTrait {
    fn update(&mut self, dt: f32);
    fn get_anim_type(&self) -> AnimationType;
    fn get_character_type(&self) -> CharacterType;
    fn get_position(&self) -> Vec2;
    fn get_actor(&self) -> Actor;
    fn get_texture(&self, textures: &Rc<CharacterTextures>) -> Rc<Texture2D>;
    fn get_facing(&self) -> Facing;
    fn get_sprite_frame(&self) -> usize;
    fn get_client_id(&self) -> Option<u64>;
    fn set_client_id(&mut self);
    fn get_size(&self) -> (i32, i32);
    fn get_velocity(&self) -> Vec2;
}
