use std::f64::consts::PI;

pub struct CrystalFrequencyGenerator {
    pub frequency: f64,
    pub crystal_type: String,
}

impl CrystalFrequencyGenerator {
    pub fn new(crystal_type: &str) -> CrystalFrequencyGenerator {
        let frequency = match crystal_type {
            "quartz" => 8_000_000.0,
            "sapphire" => 10_000_000.0,
            "graphene" => 12_000_000.0,
            _ => panic!("Invalid crystal type"),
        };

        CrystalFrequencyGenerator {
            frequency,
            crystal_type: crystal_type.to_string(),
        }
    }

    pub fn generate_sine_wave(&self) -> Vec<f64> {
        let mut sine_wave = Vec::new();
        for i in 0..1000 {
            let x = i as f64 / 1000.0 * 2.0 * PI;
            sine_wave.push(x.sin());
        }

        sine_wave
    }

    pub fn generate_frequency_sweep(&self, start_freq: f64, stop_freq: f64) -> Vec<f64> {
        let mut sweep = Vec::new();
        for i in 0..1000 {
            let freq = start_freq + (stop_freq - start_freq) * i as f64 / 1000.0;
            sweep.push(freq);
        }

        sweep
    }
}
