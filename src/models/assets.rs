#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use candid::CandidType;
use serde::{Serialize, Deserialize};

pub type AssetRef = usize;
pub type AssetKey = String;

#[repr(usize)]
#[cfg_attr(feature = "js", wasm_bindgen)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum AssetType {
    Level   = 0,
    Player  = 1,
    Npc     = 2,
    Object  = 3,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum AssetValue {
    None,
    Bool(bool),
    U32(u32),
    F32(f32),
    Text(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct AssetData(pub HashMap<AssetKey, AssetValue>);

impl AssetData {
    pub fn new(
        entries: &[(&str, &AssetValue)]
    ) -> Self {
        let mut data = HashMap::default();
        for e in entries {
            data.insert(e.0.to_string(), e.1.clone());
        }
        Self(data)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct Asset {
    pub typ: AssetType,
    pub data: AssetData,
}

impl Asset {
    pub fn new(
        typ: AssetType,
        data: AssetData
    ) -> Self {
        Self {
            typ,
            data,
        }
    }
}