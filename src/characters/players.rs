
pub trait Player {
    fn update(&mut self, dt: f32);
}

pub enum PlayerType {
    Player1,
    Player2,
}
