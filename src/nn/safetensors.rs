use std::collections::HashMap;
use serde::Deserialize;

const HEADER_SIZE_LEN: usize = 8;

#[derive(Deserialize)]
struct HeaderField {
    pub dtype: String,
    pub shape: Vec<usize>,
    pub data_offsets: [usize; 2],
}

pub struct SafeTensorData {
    pub shape: Vec<usize>,
    pub bias: Vec<f32>,
    pub weights: Vec<f32>,
}

pub struct SafeTensor {
    pub tensors: HashMap<String, SafeTensorData>,
}

impl SafeTensor {
    pub fn load(
        buf: &Vec<u8>
    ) -> Result<Self, String> {
        let header_len = u64::from_le_bytes(buf[0..HEADER_SIZE_LEN].try_into().unwrap()) as usize;
        let header_buf = &buf[HEADER_SIZE_LEN..HEADER_SIZE_LEN+header_len];
        let header = serde_json::from_slice::<HashMap<String, HeaderField>>(header_buf).unwrap();

        let mut tensors: HashMap<String, SafeTensorData> = HashMap::new();

        for e in header {
            let full_name: Vec<&str> = e.0.split('.').collect();
            let name = full_name[0].to_string();
            let comp = full_name[1];

            if e.1.dtype != "F32" {
                return Err(format!("{}.{}: Only the 'F32' data type is supported. Found: {}", name, comp, e.1.dtype));
            }

            let is_bias = match comp {
                "bias" => {
                    if e.1.shape.len() != 1 {
                        return Err(format!("{}: Shape length for 'bias' must be 1. Found: {}", name, e.1.shape.len()));
                    }
                    true
                },
                "weight" => {
                    if e.1.shape.len() != 2 {
                        return Err(format!("{}: Shape length for 'weight' must be 2. Found: {}", name, e.1.shape.len()));
                    }
                    false
                },
                _ => {
                    return Err(format!("{}: The only supported tensor components are 'bias' and 'weight'. Found: {}", name, comp));
                }
            };

            let mut off = HEADER_SIZE_LEN + header_len + e.1.data_offsets[0];
            let off_end = HEADER_SIZE_LEN + header_len + e.1.data_offsets[1];

            let tensor = if is_bias {
                let dim1 = e.1.shape[0];
                if off + dim1 * 4 != off_end {
                    return Err(format!("{}: Wrong bias length", name));
                }
                let mut bias = vec![0.0f32; dim1];
                for i in 0..dim1 {
                    bias[i] = f32::from_le_bytes([buf[off+0], buf[off+1], buf[off+2], buf[off+3]]);
                    off += 4;
                }

                SafeTensorData { 
                    shape: if tensors.contains_key(&name) {
                        tensors[&name].shape.to_owned()
                    } else {
                        vec![]
                    },
                    bias, 
                    weights: if tensors.contains_key(&name) {
                        tensors[&name].weights.to_owned()
                    } else {
                        vec![]
                    },
                }
            }
            else {
                let dim1 = e.1.shape[0];
                let dim2 = e.1.shape[1];
                if off + dim1 * dim2 * 4 != off_end {
                    return Err(format!("{}: Wrong weights length", name));
                }
                let mut weights = vec![0.0f32; dim1 * dim2];
                for i in 0..dim1*dim2 {
                    weights[i] = f32::from_le_bytes([buf[off+0], buf[off+1], buf[off+2], buf[off+3]]);
                    off += 4;
                }

                SafeTensorData { 
                    shape: vec![dim2, dim1],
                    bias: if tensors.contains_key(&name) {
                        tensors[&name].bias.to_owned()
                    } else {
                        vec![]
                    }, 
                    weights,
                }
            };

            tensors.insert(name, tensor);
        }

        Ok(Self {
            tensors
        })
    }
}