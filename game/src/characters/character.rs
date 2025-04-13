use std::rc::Rc;

use macroquad::{math::Vec2, texture::Texture2D};
use macroquad_platformer::Actor;

use common::{animation::{AnimationType, CharacterTextures, CharacterType, Facing}, types::ClientServerEvent};


pub trait CharacterTrait {
    fn update(&mut self, dt: f32);
    fn get_anim_type(&self) -> AnimationType;
    fn get_character_type(&self) -> CharacterType;
    fn get_position(&self) -> Vec2;
    fn get_size(&self) -> (i32, i32);
    fn get_actor(&self) -> Actor;
    fn get_texture(&self, textures: &Rc<CharacterTextures>) -> Rc<Texture2D>;
    fn get_facing(&self) -> Facing;
    fn get_sprite_frame(&self) -> usize;
    fn get_velocity(&self) -> Vec2;
}

pub async fn into_client_server_event(username: &str, character: &Box<dyn CharacterTrait>) -> ClientServerEvent {
    let pos = character.get_position();
    // let size = character.get_size();

    let x_pos = pos.x;
    let y_pos = pos.y;
    let anim_type = character.get_anim_type();
    let character_type = character.get_character_type();
    let sprite_frame = character.get_sprite_frame();

    ClientServerEvent {
        username: username.to_string(),
        x_pos,
        y_pos,
        facing: character.get_facing(),
        anim_type,
        character_type,
        sprite_frame,
    }
}
