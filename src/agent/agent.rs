use rand::rngs::StdRng;
use crate::nn::tensor::Tensor;
use super::baseagent::BaseAgentOptions;

pub trait Agent {
    fn new(
        options: &BaseAgentOptions
    ) -> Self where Self: Sized;

    fn choose_action(
        &mut self, 
        obs: Tensor,
        rng: &mut StdRng
    ) -> Option<usize>;

    fn load(
        &mut self,
        buf: &Vec<u8>
    ) -> Result<(), String>;
}