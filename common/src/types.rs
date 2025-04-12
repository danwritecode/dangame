use std::collections::HashMap;

use crate::animation::{AnimationType, CharacterType, Facing};
use bincode::{Decode, Encode};

pub type ClientId = u64;
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
    NewClient(HashMap<UserNameText, ClientId>),
    ClientCharacterUpdate(HashMap<ClientId, ClientServerEvent>),
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ClientServerEvent {
    /// The server gives us the client_id
    pub client_id: u64,

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

    /// Sequence Index is the current sequence of animations which has a number of frames and an FPS to play them at.
    pub sequence_index: usize,

    /// Sequence Frame Index is the current frame of the sequence
    pub sequence_frame_index: usize,
}


/// ClientState is used to get the current generic state of a character
/// We then convert that to a ClientServerEvent to send to the server
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct ClientState {
    x_pos: f32,
    y_pos: f32,
    pub facing: Facing,

    // this determines the texture
    pub anim_type: AnimationType,
    pub character_type: CharacterType,

    // these determine the frame being played
    // it is not our clients job to keep track of other peoples frames
    // we just need to know which to render

    /// Frame is the current frame of the spritesheetNOT the animation frame
    pub sprite_frame: usize,

    /// Sequence Index is the current sequence of animations which has a number of frames and an FPS to play them at.
    pub sequence_index: usize,

    /// Sequence Frame Index is the current frame of the sequence
    pub sequence_frame_index: usize,
}

impl ClientState {
    pub fn new(
        x_pos: f32,
        y_pos: f32,
        facing: Facing,
        anim_type: AnimationType,
        character_type: CharacterType,
        sprite_frame: usize,
        sequence_index: usize,
        sequence_frame_index: usize,
    ) -> Self {
        Self {
            x_pos,
            y_pos,
            facing,
            anim_type,
            character_type,
            sprite_frame,
            sequence_index,
            sequence_frame_index,
        }
    }

    fn into_client_server_event(self, client_id: u64, username: String) -> ClientServerEvent {
        ClientServerEvent {
            client_id,
            username,
            x_pos: self.x_pos,
            y_pos: self.y_pos,
            facing: self.facing,
            anim_type: self.anim_type,
            character_type: self.character_type,
            sprite_frame: self.sprite_frame,
            sequence_index: self.sequence_index,
            sequence_frame_index: self.sequence_frame_index,
        }
    }
}
