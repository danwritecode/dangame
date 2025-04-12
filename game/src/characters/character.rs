use std::rc::Rc;

use macroquad::{math::Vec2, texture::Texture2D};
use macroquad_platformer::Actor;

use common::{animation::{CharacterTextures, Facing}, types::ClientState};


pub trait CharacterTrait {
    fn update(&mut self, dt: f32);
    fn get_actor(&self) -> Actor;
    fn get_texture(&self, textures: &Rc<CharacterTextures>) -> Rc<Texture2D>;
    fn get_facing(&self) -> Facing;
    fn get_sprite_frame(&self) -> usize;
    fn get_velocity(&self) -> Vec2;
    fn get_client_state(&self) -> ClientState;
}
