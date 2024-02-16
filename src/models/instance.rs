use std::collections::HashMap;
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use super::assets::{Asset, AssetRef};

pub type InstanceId = String;

pub type InstanceOptions = HashMap<String, String>;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceAssetsResponse {
    pub assets: HashMap::<AssetRef, Asset>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceStep {
    pub terminated: bool,
    pub timedout: bool,
    pub events: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceStepsResponse {
    pub steps: Vec<InstanceStep>,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct InstanceRunResponse {
    pub won_at: Option<u64>,
    pub won_by: Option<Principal>,
}
