use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ActivationFnType {
    RELU,
}

pub type ActivationFn = dyn Fn(f32) -> f32;

#[derive(Clone)]
pub struct Activation {
}

impl Activation {
    pub fn nop(
        v: f32
    ) -> f32 {
        v
    }

    pub fn make_nop(
    ) -> &'static ActivationFn {
        &Self::nop
    }

    pub fn relu(
        v: f32
    ) -> f32 {
        if v >= 0.0 {
            v
        } else {
            0.0
        }
    }

    pub fn make_relu(
    ) -> &'static ActivationFn {
        &Self::relu    }
    
}