extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod spin;
mod exchange_matrix;
mod monte_carlo;
mod plot;
mod lattice;
mod point;
mod energy;
mod simulated_annealing;

use spin::{Spin, SpinConfiguration, IsingSpinConfiguration};
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;
use lattice::{Site,Lattice};
use point::Point;
use spin::IsingSpin;
use energy::{calculate_ising_energy, average};


fn main()
{
//    let mut ising_spins:IsingSpinConfiguration = MonteCarlo::run_ising_monte_carlo(4, 4, 10000, 10000, 5.0);
//    println!("...");
//    println!("...");
//    println!("Final Ising spin configuration:");
//    &ising_spins.print_spin_configuration();
//    println!("The final energy is: {}", calculate_ising_energy(&mut ising_spins));
//
//
//    let mut numbers = [42.0, 1.0, 36.0, 34.0, 76.0, 378.0, 43.0, 1.0, 43.0, 54.0, 2.0, 3.0, 43.0];
//
//    println!("AVERAGE: {}", average(&numbers));

    let simulation =  MonteCarlo::metrolpolis_algorithm(20, 20);
    println!("{:?}",simulation);
}