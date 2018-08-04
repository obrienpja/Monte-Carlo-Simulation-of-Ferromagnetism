use monte_carlo::MonteCarlo;
use spin::{SpinConfiguration};

struct SimulatedAnnealing{
    spin_configuration: SpinConfiguration,
    average_energy: Vec<f64>
}

//impl SimulatedAnnealing{
//    pub fn run_simulated_annealing() -> (){
//        for temp in 1..100{
////            MonteCarlo::run_ising_monte_carlo();
//            println!("{}", 4.25 - temp);
//        }
//    }
//
//    pub fn metropolis_algorithm() -> (){
//
//    }
//}