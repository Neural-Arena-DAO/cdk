use serde::{Deserialize, Serialize};
use super::layer::Layer;
use super::tensor::Tensor; 
use super::linear::Linear;
use super::safetensors::SafeTensor; 
use super::activation::{Activation, ActivationFnType};

#[derive(Clone, Serialize, Deserialize)]
pub struct Sequential<L> 
    where L: Layer {
    pub layers: Vec<L>,
    pub funs: Vec<ActivationFnType>,
}

impl Sequential<Linear> {
    pub fn add(
        &mut self,
        layer: Linear
    ) {
        self.layers.push(layer);
    }

    pub fn add_fn(
        &mut self,
        fun: ActivationFnType
    ) {
        self.funs.push(fun);
    }

    pub fn apply(
        &self, 
        input: Tensor
    ) -> Tensor {
        let nop = Activation::make_nop();
        let relu = Activation::make_relu();
        
        let mut inp = input.0;
        unsafe {
            for l in 0..self.layers.len() {
                let layer = &self.layers[l];
                if inp.len() != layer.in_dim {
                    panic!("Input dimensions don't match: {}x{}", layer.in_dim, inp.len());
                }
                
                let ninp = layer.in_dim;
                let nout = layer.out_dim;
                
                let out = &mut vec![0.0f32; nout];
                let act = if l < self.funs.len() {
                    match self.funs[l] {
                        ActivationFnType::RELU => relu
                    }
                } 
                else {
                    nop
                };

                for i in 0..nout {
                    let row = i * ninp;
                    let mut sum = 0.0f32;
                    for j in 0..ninp {
                        sum += inp.get_unchecked(j) * layer.weights.get_unchecked(row + j);
                    };
                    *out.get_unchecked_mut(i) = act(sum + layer.bias.get_unchecked(i));
                }
                
                inp = out.to_owned();
            }
        }

        Tensor(inp)
    }

    pub fn load(
        &mut self,
        buf: &Vec<u8>
    ) -> Result<(), String> {
        let st = SafeTensor::load(buf)?;

        for src in st.tensors {
            let dst = self.layers.iter_mut().find(|l| l.name == src.0);
            if let Some(dst) = dst {
                if dst.bias.len() != src.1.bias.len() {
                    return Err(format!("{}: Bias lengths don't match: {}x{}", src.0, dst.bias.len(), src.1.bias.len()));
                }
                dst.bias.copy_from_slice(&src.1.bias);

                if dst.weights.len() != src.1.weights.len() {
                    return Err(format!("{}: Weights lengths don't match: {}x{}", src.0, dst.weights.len(), src.1.weights.len()));
                }

                if src.1.shape.len() != 2 {
                    return Err(format!("{}: Shape length for tensor must be 2. Found {}", src.0, src.1.shape.len()));
                }

                if src.1.shape[0] != dst.in_dim {
                    return Err(format!("{}: Input dimensions don't match: {}x{}", src.0, dst.in_dim, src.1.shape[0]));
                }
                else if src.1.shape[1] != dst.out_dim {
                    return Err(format!("{}: Output dimensions don't match: {}x{}", src.0, dst.out_dim, src.1.shape[1]));
                }

                dst.weights.copy_from_slice(&src.1.weights);
            }
            else {
                return Err(format!("Unknown layer: {}", src.0));
            }
        }

        Ok(())
    }
}

pub fn seq<L> (
) -> Sequential<L> 
    where L: Layer {
    Sequential::<L> {  
        layers: vec![],
        funs: vec![],
    }
}
