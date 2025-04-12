use crate::animation::{AnimationType, CharacterType, Facing};
use bincode::{Decode, Encode};


pub enum ClientServerEventType {
    NewClient(ClientServerEventNewClient),
    ClientCharacterUpdate(ClientServerEvent),
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct ClientServerEventNewClient {
    pub client_id: u64,
    x_pos: f32,
    y_pos: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct ClientServerEvent {
    pub client_id: u64,
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

impl ClientServerEvent {
    pub fn new(
        client_id: u64,
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
            client_id,
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
}


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
}
