use crate::character::CharacterTrait;
use bincode::{config, Decode, Encode};


type PosDelta = (f32, f32);
type VelDelta = (f32, f32);

pub struct UpdateDeltas {
    pub pos_delta: PosDelta,
    pub vel_delta: VelDelta,
    pub height: i32,
    pub width: i32,
}

impl Default for UpdateDeltas {
    fn default() -> Self {
        Self { pos_delta: (0.0, 0.0), vel_delta: (0.0, 0.0), height: 0, width: 0 }
    }
}



pub enum ServerEvent {
    NewClient { client_id: u64, character: Box<dyn CharacterTrait> },
    ClientCharacterUpdate { client_id: u64, character: Box<dyn CharacterTrait> },
}

pub struct ClientEvent {
    pub client_id: u64,
    pub character: Box<dyn CharacterTrait>,
}
