use internal::teleportation_coordinator::TeleportationCoordinator;
use internal::user_interface::UserInterface;

fn main() {
    // Create an instance of the teleportation coordinator
    let mut teleportation_coordinator = TeleportationCoordinator::new();

    // Create an instance of the user interface
    let mut user_interface = UserInterface::new();

    // Start the teleportation process
    teleportation_coordinator.start();

    // Display the results through the user interface
    user_interface.display_results(teleportation_coordinator.get_results());
}
