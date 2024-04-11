use std::collections::HashMap;
use candid::CandidType;
use serde::{Serialize, Deserialize};

pub type GameMapId = usize;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameMapMusicUrls {
    pub main: String,
    pub victory: String,
    pub over: String,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameMapUrls {
    pub model: String,
    pub image: String,
    pub music: GameMapMusicUrls,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameMap {
    pub id: GameMapId,
    pub name: String,
    pub description: String,
    pub version: f32,
    pub urls: GameMapUrls,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameSceneURLs {
    pub model: String,
    pub music: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameURLs {
    pub cover: String,
    pub engine_wasm: String,
    pub assets: String,
    pub scenes: HashMap<String, GameSceneURLs>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub enum GameCost {
    USD(u64),           // in e8s (* 100_000_000)
    ICP(u128),          // in e8s (* 100_000_000)
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameCosts {
    pub to_play: GameCost,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameSfx {
    pub url: String,
    pub repeat: bool,
    pub rate: Option<f32>,
    pub detune: Option<f32>,
}
#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameRewards {
    pub winner: u128,   // in e2s (* 100)
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct GameInfo {
    pub name: String,
    pub subtitle: String,
    pub description: String,
    pub version: f32,
    pub category: String,
    pub fps: u32,
    pub min_players: usize,
    pub max_players: usize,
    pub costs: GameCosts,
    pub rewards: GameRewards,
    pub urls: GameURLs,
    pub skills: Vec<String>,
    pub states: Vec<String>,
    pub sfx_sets: Vec<HashMap::<String, GameSfx>>,
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
