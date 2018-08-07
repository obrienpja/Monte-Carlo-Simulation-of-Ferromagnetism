use spin::{SpinConfiguration};

struct SimulatedAnnealing{
    spin_configuration: SpinConfiguration,
    average_energy: Vec<f64>
}