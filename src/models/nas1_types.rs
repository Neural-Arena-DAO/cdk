use std::collections::HashMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub const MIN_XP: f32 = 0.0;
pub const MAX_XP: f32 = 100.0;
pub const MIN_GAME_PROPS_DELTA: f32 = 0.1;

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub enum Nas1Value {
    Nat(u128),
    Int(i128),
    Float(f32),
    Double(f64),
    Text(String),
    Blob(Vec<u8>),
    Array(Vec<Nas1Value>),
    Map(HashMap<String, Nas1Value>),
}

impl Nas1Value {
    pub fn as_nat(
        &self
    ) -> Option<u128> {
        match self {
            Nas1Value::Nat(value) => 
                Some(*value),
            Nas1Value::Int(value) => 
                Some(*value as _),
            _ => None
        }
    }

    pub fn as_int(
        &self
    ) -> Option<i128> {
        match self {
            Nas1Value::Nat(value) => 
                Some(*value as _),
            Nas1Value::Int(value) => 
                Some(*value),
            _ => None
        }
    }

    pub fn as_f32(
        &self
    ) -> Option<f32> {
        match self {
            Nas1Value::Float(value) => 
                Some(*value),
            Nas1Value::Double(value) => 
                Some(*value as _),
            _ => None
        }
    }

    pub fn as_f64(
        &self
    ) -> Option<f64> {
        match self {
            Nas1Value::Float(value) => 
                Some(*value as _),
            Nas1Value::Double(value) => 
                Some(*value),
            _ => None
        }
    }

    pub fn as_string(
        &self
    ) -> Option<String> {
        match self {
            Nas1Value::Text(value) => 
                Some(value.clone()),
            _ => None
        }
    }

    pub fn as_string_array(
        &self
    ) -> Option<Vec<String>> {
        match self {
            Nas1Value::Array(arr) => {
                let mut res = vec![];
                for item in arr {
                    if let Some(s) = item.as_string() {
                        res.push(s);
                    }
                    else {
                        return None;
                    }
                }
                Some(res)
            },
            _ => None
        }
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1TokenGameProps {
    pub xp: f32,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1TokenGameDeltaProps {
    pub delta_xp: f32,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1Token {
    pub name: String,
    pub image_url: String,
    pub model: String,
    pub model_url: String,
    pub sfx_set: usize,
    pub height: f32,
    pub radius: f32,
    pub epsilon: f32,
    pub num_hidden_layers: usize,
    pub hidden_layers_weights_size: Vec<usize>,
    pub steps_per_thought: usize,
    pub traits: Vec<String>,
    pub extra: HashMap<String, Nas1Value>,
    pub game_props: HashMap<Principal, Nas1TokenGameProps>,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1Sfx {
    pub url: String,
    pub repeat: bool,
    pub rate: Option<f32>,
    pub detune: Option<f32>,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1Collection {
    pub version: f32,
    pub subtitle: String,
    pub assets_canister_id: Principal,
    pub assets_url: String,
    pub states: Vec<String>,
    pub skills: Vec<String>,
    pub sfx_sets: Vec<HashMap<String, Nas1Sfx>>,
}

impl Default for Nas1Collection {
    fn default() -> Self {
        Self { 
            version: 0.0,
            subtitle: "".to_string(),
            assets_canister_id: Principal::anonymous(), 
            assets_url: Default::default(),
            states: vec![],
            skills: vec![],
            sfx_sets: vec![],
        }
    }
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Nas1UpdatePropsArg {
    pub token_id: u128,
    pub game_canister_id: Principal,
    pub game_props: Nas1TokenGameDeltaProps,
}
