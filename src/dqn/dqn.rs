use serde::{Deserialize, Serialize};
use crate::nn::{
    sequential::{Sequential, seq}, 
    tensor::Tensor, 
    linear::{linear, Linear}, 
    activation::ActivationFnType
};

#[derive(Clone, Serialize, Deserialize)]
pub struct DqnOptions {
    pub n_obs: usize,
    pub n_hidden: usize,
    pub n_actions: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeepQNetwork {
    pub options: DqnOptions,
    pub network: Sequential<Linear>,
}

impl DeepQNetwork {
    pub fn new(
        options: &DqnOptions
    ) -> Self {

        let mut network = seq();
        
        network.add(linear(
            "in-hid1",
            options.n_obs, 
            options.n_hidden, 
            Default::default()
        ));

        network.add_fn(ActivationFnType::RELU);

        network.add(linear(
            "hid1-out",
            options.n_hidden, 
            options.n_actions, 
            Default::default()
        ));

        Self {
            network,
            options: options.clone(),
        }
    }

    pub fn forward(
        &self, 
        obs: Tensor
    ) -> Tensor {
        self.network.apply(obs)
    }

    pub fn load(
        &mut self,
        buf: &Vec<u8>
    ) -> Result<(), String> {
        self.network.load(buf)
    }
}