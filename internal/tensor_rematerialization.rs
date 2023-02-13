use ndarray::Array2;

pub struct TensorRematerialization {
    rematerialized_tensors: Vec<Array2<f64>>,
}

impl TensorRematerialization {
    pub fn new() -> Self {
        TensorRematerialization {
            rematerialized_tensors: vec![],
        }
    }

    pub fn rematerialize_tensor(&mut self, tensor: Array2<f64>) {
        self.rematerialized_tensors.push(tensor);
    }

    pub fn get_rematerialized_tensors(&self) -> &Vec<Array2<f64>> {
        &self.rematerialized_tensors
    }
}
