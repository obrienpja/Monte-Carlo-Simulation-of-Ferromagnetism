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

use spin::{Spin, SpinConfiguration, IsingSpinConfiguration};
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;
use lattice::{Site,Lattice};
use point::Point;
use spin::IsingSpin;
use energy::calculate_ising_energy;


fn main()
{
    let mut ising_spins:IsingSpinConfiguration = MonteCarlo::run_ising_monte_carlo(10000, 4.0);
    println!("...");
    println!("...");
    println!("Final Ising spin configuration:");
    &ising_spins.print_spin_config();
    println!("The final energy is: {}", calculate_ising_energy(&mut ising_spins));
}