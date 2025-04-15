use std::{cell::RefCell, rc::Rc};

use common::animation::{AnimationType, CharacterTextures, CharacterType, Facing};
use macroquad::{math::vec2, texture::Texture2D};
use macroquad_platformer::{Actor, World};


pub struct ServerCharacter {
    pub x_pos: f32,
    pub y_pos: f32,
    pub height: i32,
    pub width: i32,
    pub facing: Facing,
    pub anim_type: AnimationType,
    pub character_type: CharacterType,
    pub sprite_frame: usize,
    pub actor: Actor,
    pub world: Rc<RefCell<World>>,
}

/// This is a lightweight version of the Character that the local player uses to render
impl ServerCharacter {
    pub async fn new(
        x_pos: f32, 
        y_pos: f32, 
        height: i32, 
        width: i32, 
        facing: Facing,
        anim_type: AnimationType,
        character_type: CharacterType,
        sprite_frame: usize,
        world: Rc<RefCell<World>>
    ) -> Self {
        let actor = world
            .borrow_mut()
            .add_actor(vec2(x_pos, y_pos), width as i32, height as i32);

        Self {
            x_pos,
            y_pos,
            height,
            width,
            facing,
            anim_type,
            character_type,
            sprite_frame,
            actor,
            world,
        }
    }

    pub fn update(&mut self) {
        let pos = vec2(self.x_pos, self.y_pos);
        self.world.borrow_mut().set_actor_position(self.actor, pos);
        self.world.borrow_mut().set_actor_size(self.actor, self.width, self.height);
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
}
