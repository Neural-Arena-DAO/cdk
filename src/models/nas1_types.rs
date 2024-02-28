use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1Collection {
    pub assets_canister_id: Principal,
    pub assets_url: String,
    pub animations: Vec<String>,
    pub skills: Vec<String>,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1Token {
    pub xp: usize,
    pub model: String,
    pub radius: f32,
    pub height: f32,
    pub traits: Vec<String>,
}
