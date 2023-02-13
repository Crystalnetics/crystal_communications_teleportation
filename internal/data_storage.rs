use std::collections::HashMap;
use ndarray::Array2;

pub struct DataStorage {
    data: HashMap<String, Array2<f64>>,
}

impl DataStorage {
    pub fn new() -> Self {
        DataStorage {
            data: HashMap::new(),
        }
    }

    pub fn store_data(&mut self, key: String, value: Array2<f64>) {
        self.data.insert(key, value);
    }

    pub fn retrieve_data(&self, key: &str) -> Option<&Array2<f64>> {
        self.data.get(key)
    }
}
