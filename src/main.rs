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

use spin::SpinConfig;
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;
use lattice::Lattice;
use point::Point;


fn main()
{
    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);
    println!("{}", (ferro_exchange.exchange_matrix.data().len() as f64).sqrt() as usize);

    let mut test2: SpinConfig = MonteCarlo::run_monte_carlo(100000, &mut ferro_exchange);

//    println!("The energy of the final spin configuration is: {}",
//             ferro_exchange.calculate_energy(test2));

    test2.print_spin_config(10);

//    let lat = Lattice::new();
    let points = Lattice::generate_square_lattice();
//    println!("{}", points.lattice[13]);

    println!("{}", ExchangeMatrix::phase_factor(Point { x: 1.0, y: 1.0, z: 1.0 }, Point { x: 3.1, y: 1.0, z: 1.0 }));
}