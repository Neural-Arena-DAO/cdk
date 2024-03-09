use candid::CandidType;
use serde::{Serialize, Deserialize};

pub type GameMapId = usize;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameMap {
    pub id: GameMapId,
    pub name: String,
    pub description: String,
    pub version: f32,
    pub image_url: String,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameInfo {
    pub name: String,
    pub description: String,
    pub version: f32,
    pub category: String,
    pub image_url: String,
    pub min_players: usize,
    pub max_players: usize,
    pub skills: Vec<String>,
    pub animations: Vec<String>,
    pub engine_wasm_url: String,
    pub assets_url: String,
    pub maps: Vec<GameMap>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameInfoResponse {
    pub info: GameInfo,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameEndCredits {
    pub title: String,
    pub people: Vec<(String, String)>,
    pub sub: Vec<GameEndCredits>,
}
