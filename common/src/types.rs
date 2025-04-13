use std::collections::HashMap;

use crate::animation::{AnimationType, CharacterType, Facing};
use bincode::{Decode, Encode};

pub type UserNameText = String;

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
    ClientCharacterUpdate(HashMap<UserNameText, ClientServerEvent>),
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ClientServerEvent {
    /// We give this to the server
    pub username: String,

    pub x_pos: f32,
    pub y_pos: f32,
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
