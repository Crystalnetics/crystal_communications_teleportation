use std::fmt::Display;

pub struct UserInterface {}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {}
    }

    pub fn display_results<T: Display>(&self, results: T) {
        println!("Results: {}", results);
    }
}
