extern crate rand;

mod spin;
mod exchange_matrix;
mod monte_carlo;

use spin::SpinConfig;
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;

fn main()
{

    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);

    let test2:SpinConfig = MonteCarlo::run_monte_carlo(1000);

    println!("The energy of the final spin configuration is: {}", ferro_exchange.calculate_energy(test2));
}