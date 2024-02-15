#[derive(Clone)]
pub struct Tensor(pub Vec<f32>);

impl Tensor {
    pub fn argmax(
        &self
    ) -> usize {
        self.0.iter()
            .enumerate()
            .fold(
                (usize::MAX, &f32::MIN),
                |acc, e| if *acc.1 > *e.1 {
                    acc   
                }
                else {
                    e    
                })
            .0
    }

    pub fn _relu(
        &self
    ) -> Self {
        Self(
            self.0.iter()
            .map(|v| if *v >= 0.0 {
                    *v
                } else {
                    0.0
                })
            .collect()
        )
    }
}

impl From<Vec<f32>> for Tensor {
    fn from(
        value: Vec<f32>
    ) -> Self {
        Tensor(value)
    }
}

impl From<&Vec<f32>> for Tensor {
    fn from(
        value: &Vec<f32>
    ) -> Self {
        Tensor(value.clone())
    }
}