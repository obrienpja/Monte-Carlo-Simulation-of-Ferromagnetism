extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate num;

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
use monte_carlo::run_ising_monte_carlo;
use energy::magnetization;
use num::abs;


fn main()
{
    let simulation =  MonteCarlo::metrolpolis_algorithm(20, 20);
    println!("{:?}", simulation);
}