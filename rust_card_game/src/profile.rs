use serde::{Deserialize, Serialize};

#[dervie()]
pub struct PlayerState {
    pub wins: u32,
    pub losses: u32,
    pub pushes: u32,
}