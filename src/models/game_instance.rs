use std::collections::HashMap;
use candid::CandidType;
use serde::{Serialize, Deserialize};
use super::assets::{Asset, AssetRef};

pub type InstanceId = String;

pub type InstanceOptions = HashMap<String, String>;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceCreateResponse {
    pub instance_id: InstanceId,
    pub min_players: usize,
    pub max_players: usize,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceAssetsResponse {
    pub assets: HashMap::<AssetRef, Asset>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceStep {
    pub terminated: bool,
    pub timedout: bool,
    pub entities: Vec<u8>,
    pub events: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceStepsResponse {
    pub steps: Vec<InstanceStep>,
}