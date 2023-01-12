use crate::internal::quantum_teleporter::QuantumTeleporter;
use crate::internal::crystal_frequency_generator::CrystalFrequencyGenerator;
use crate::internal::tensor_rematerialization::TensorRematerialization;

pub struct TeleportationCoordinator {
    quantum_teleporter: QuantumTeleporter,
    crystal_frequency_generator: CrystalFrequencyGenerator,
    tensor_rematerialization: TensorRematerialization,
}

impl TeleportationCoordinator {
    pub fn new() -> Self {
        let quantum_teleporter = QuantumTeleporter::new();
        let crystal_frequency_generator = CrystalFrequencyGenerator::new();
        let tensor_rematerialization = TensorRematerialization::new();

        Self {
            quantum_teleporter,
            crystal_frequency_generator,
            tensor_rematerialization,
        }
    }

    pub fn teleport(&self, tensor: &Tensor) {
        // Step 1: Generate the crystal frequency
        let crystal_frequency = self.crystal_frequency_generator.generate();

        // Step 2: Perform quantum teleportation
        self.quantum_teleporter.teleport(tensor, crystal_frequency);

        // Step 3: Rematerialize the tensor
        self.tensor_rematerialization.rematerialize(tensor);
    }
}
