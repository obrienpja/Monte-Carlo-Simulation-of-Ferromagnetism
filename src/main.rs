//extern crate rand;

extern crate gnuplot;
mod spin;
mod exchange_matrix;
mod monte_carlo;
mod lattice;
mod point;
mod plot;
mod energy;

use spin::SpinConfig;
use exchange_matrix::ExchangeMatrix;
use monte_carlo::MonteCarlo;
use lattice::Lattice;
use point::Point;
use energy::boltzmann_factor;


fn main()
{
//    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);
//    println!("{}",(ferro_exchange.exchange_matrix.data().len() as f64).sqrt() as usize);
//
//    let mut test2:SpinConfig = MonteCarlo::run_monte_carlo(100000, &mut ferro_exchange);
//
//    test2.print_spin_config(10);
//    let points = Lattice::generate_square_lattice();
//
//
//    println!("{}", ExchangeMatrix::phase_factor(Point{x:1.0, y:1.0, z:1.0}, Point{x:3.1, y:1.0, z:1.0}));

    plot::plot(1.1, 2.2, 3.3);

    println!("{}", boltzmann_factor(-0.5,0.5))
}