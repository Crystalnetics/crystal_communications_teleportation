use ndarray::Array2;

pub struct TensorFlightDynamics {
    flight_paths: Vec<Array2<f64>>,
}

impl TensorFlightDynamics {
    pub fn new() -> Self {
        TensorFlightDynamics {
            flight_paths: vec![],
        }
    }

    pub fn simulate_flight(&mut self, tensor: Array2<f64>) {
        // Perform simulation of the flight of the tensor
        // ...

        // Add the flight path to the flight_paths vector
        self.flight_paths.push(tensor);
    }

    pub fn get_flight_paths(&self) -> &Vec<Array2<f64>> {
        &self.flight_paths
    }
}
