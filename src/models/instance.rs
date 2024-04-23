use std::collections::HashMap;
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;
use super::{
    assets::{Asset, AssetRef}, 
    nas1_types::Nas1Token, 
    nft::NftCollectionMetadata
};

pub type InstanceId = String;

pub type InstanceOptions = HashMap<String, String>;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstancePlayerState {
    pub active: bool,
    pub last_thought: usize,
    pub score: f32,
    pub health: f32,
    pub xp: f32,
    pub obs: Vec<f32>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstancePlayerNft {
    pub collection_id: Principal,
    pub token_id: u128,
    pub props: Option<Nas1Token>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstancePlayer {
    pub id: Principal,
    pub nft_col: NftCollectionMetadata,
    pub nft: InstancePlayerNft,
    pub tensors: HashMap<String, Vec<u8>>,
    pub state: InstancePlayerState,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceAssetsResponse {
    pub assets: HashMap::<AssetRef, Asset>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceStep {
    pub events: Vec<u8>,
    pub entities: Vec<u8>,
}

#[cfg_attr(feature = "js", wasm_bindgen)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub struct InstancePlayerEntity {
    pub health: f32,
    pub score: f32,
    pub xp: f32,
}

#[cfg_attr(feature = "js", wasm_bindgen)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub struct InstanceNpcEntity {
    pub health: f32,
}

#[cfg_attr(feature = "js", wasm_bindgen)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, CandidType)]
pub struct InstanceObjectEntity {
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct InstanceEntities {
    pub players: Vec<InstancePlayerEntity>,
    pub npcs: Vec<InstanceNpcEntity>,
    pub objects: Vec<InstanceObjectEntity>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceStepsResponse {
    pub steps: Vec<InstanceStep>
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceRunRequest {
    pub slice: usize,
    pub seed: u64,
    pub max_instructions: u64,
    pub max_steps: usize,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceRunSlice {
    pub first_step: usize,
    pub seed: u64,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceRunResponse {
    pub terminated: bool,
    pub timedout: bool,
    pub steps: usize,
    pub players: Vec<InstancePlayerEntity>,
    pub won_at: Option<u64>,
    pub won_by: Option<Principal>,
}
