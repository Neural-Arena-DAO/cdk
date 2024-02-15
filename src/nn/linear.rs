use serde::{Deserialize, Serialize};
use super::layer::Layer;

#[derive(Clone, Serialize, Deserialize)]
pub struct Linear {
    pub name: String,
    pub in_dim: usize,
    pub out_dim: usize,
    pub bias: Vec<f32>,
    pub weights: Vec<f32>,
}

impl Layer for Linear {
    
}

pub fn linear(
    name: &str,
    in_dim: usize,
    out_dim: usize,
    default: f32
) -> Linear {
    Linear {  
        name: name.to_string(),
        in_dim,
        out_dim,
        bias: vec![default; out_dim],
        weights: vec![default; in_dim * out_dim],
    }
}