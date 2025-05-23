use std::collections::HashMap;

use crate::{animation::{AnimationType, CharacterType, Facing}, constants::{DEFAULT_PLAYER_HEIGHT, DEFAULT_PLAYER_WIDTH}};
use bincode::{Decode, Encode};

/// This is what gets sent to the server AND the client.
///
/// When clients connect to the server, they send a ClientEventNewClient.
///
/// The server then sends NewClient events to all clients.
///
/// When a client updates their character state, they send a ClientServerEvent.
///
/// The server then sends ClientServerEvent to all clients.
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum ClientEventType {
    ClientCharacterUpdate(HashMap<u64, ServerClient>),
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ServerClient {
    pub x_pos: f32,
    pub y_pos: f32,

    pub prev_x_pos: f32,
    pub prev_y_pos: f32,

    pub height: i32,
    pub width: i32,

    pub facing: Facing,

    // this determines the texture
    pub anim_type: AnimationType,
    pub character_type: CharacterType,

    // these determine the frame being played
    // it is not our clients job to keep track of other peoples frames
    // we just need to know which to render

    /// Frame is the current frame of the spritesheetNOT the animation frame
    pub sprite_frame: usize,
}

impl Default for ServerClient {
    fn default() -> Self {
        Self {
            x_pos: 0.0,
            y_pos: 0.0,
            prev_x_pos: 0.0,
            prev_y_pos: 0.0,
            height: DEFAULT_PLAYER_HEIGHT,
            width: DEFAULT_PLAYER_WIDTH,
            facing: Facing::Right,
            anim_type: AnimationType::Idle,
            character_type: CharacterType::Fighter,
            sprite_frame: 0,
        }
    }
}
