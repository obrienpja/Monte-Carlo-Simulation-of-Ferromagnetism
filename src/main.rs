//extern crate rand;

mod spin;
mod exchange_matrix;
mod monte_carlo;
mod lattice;
mod point;

use spin::SpinConfig;
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;
use lattice::Lattice;
use point::Point;


fn main()
{
    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);

    let mut test2:SpinConfig = MonteCarlo::run_monte_carlo_ferro(100000);

//    println!("The energy of the final spin configuration is: {}",
//             ferro_exchange.calculate_energy(test2));

    test2.print_spin_config(10);

//    let lat = Lattice::new();
    let points:Vec<Point> = Lattice::generate_square_lattice();
    println!("{}", points[13])
}