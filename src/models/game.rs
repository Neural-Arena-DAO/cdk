use candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameInfo {
    pub name: String,
    pub description: String,
    pub version: f32,
    pub category: String,
    pub min_players: usize,
    pub max_players: usize,
    pub skills: Vec<String>,
    pub animations: Vec<String>,
    pub assets_url: String,
    pub engine_wasm_url: String,
    pub image_url: String,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameInfoResponse {
    pub info: GameInfo,
}
