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

use spin::{Spin, SpinConfig};
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;
use lattice::Lattice;
use point::Point;
use spin::IsingSpin;


fn main()
{
    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);
    println!("{}", (ferro_exchange.exchange_matrix.data().len() as f64).sqrt() as usize);

    let mut test2: SpinConfig = MonteCarlo::run_monte_carlo(100000, &mut ferro_exchange);

//    println!("The energy of the final spin configuration is: {}",
//             ferro_exchange.calculate_energy(test2));

    test2.print_spin_config();

//    let lat = Lattice::new();
    let points = Lattice::generate_1d_lattice();
//    println!("{}", points.lattice[13]);

    println!("{}", ExchangeMatrix::phase_factor(Point { x: 1.0, y: 1.0, z: 1.0 }, Point { x: 3.1, y: 1.0, z: 1.0 }));
    println!("{}", Spin::create_random_ising_spin());

//    for i in 0..16{
//        Lattice::map_to_site(i,4);
//    }
    for i in 0..4{
        println!("{}", Lattice::neighbor_list(0,1,4,4)[i])
    }
}