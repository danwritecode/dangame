use std::{cell::RefCell, rc::Rc};

use common::animation::{AnimationType, CharacterTextures, CharacterType, Facing};
use macroquad::{math::vec2, texture::Texture2D};
use macroquad_platformer::{Actor, World};


pub struct ServerCharacter {
    pub x_v: f32,
    pub y_v: f32,
    pub facing: Facing,
    pub anim_type: AnimationType,
    pub character_type: CharacterType,
    pub sprite_frame: usize,
    pub actor: Actor,
    pub world: Rc<RefCell<World>>,
}

impl ServerCharacter {
    pub async fn new(
        x: f32, 
        y: f32, 
        width: i32, 
        height: i32, 
        anim_type: AnimationType,
        character_type: CharacterType,
        sprite_frame: usize,
        world: Rc<RefCell<World>>
    ) -> Self {
        let actor = world
            .borrow_mut()
            .add_actor(vec2(x, y), width as i32, height as i32);

        Self {
            x_v: 0.0,
            y_v: 0.0,
            facing: Facing::Right,
            anim_type,
            character_type,
            sprite_frame,
            actor,
            world,
        }
    }

    pub fn get_texture(&self, textures: &Rc<CharacterTextures>) -> Rc<Texture2D> {
        let texture = textures.get_texture(
            &self.character_type,
            &self.anim_type,
        );
        Rc::clone(&texture)
    }

    pub fn get_actor(&self) -> Actor {
        self.actor
    }

    pub fn get_facing(&self) -> Facing {
        self.facing.clone()
    }

    pub fn get_sprite_frame(&self) -> usize {
        self.sprite_frame
    }

    pub fn get_character_type(&self) -> CharacterType {
        self.character_type.clone()
    }

    pub fn get_anim_type(&self) -> AnimationType {
        self.anim_type.clone()
    }
}
