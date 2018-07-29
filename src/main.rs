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
use lattice::{Site,Lattice};
use point::Point;
use spin::IsingSpin;


fn main()
{
//    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);
//    println!("{}", (ferro_exchange.exchange_matrix.data().len() as f64).sqrt() as usize);
//
//    let mut test2: SpinConfig = MonteCarlo::run_monte_carlo(100000, &mut ferro_exchange);
//
//    test2.print_spin_config();

    let mut points = Lattice::generate_square_lattice(4,4);

    println!("Square lattice coordinates: ");
    points.print_lattice();

    let mut ising_spins:SpinConfig = SpinConfig::random_ising_spin_config(16);
    println!("Ising spins: ");
    ising_spins.print_spin_config();

    let j = 1.0;
    let n_x_size = 4;
    let n_y_size = 4;
//    println!("Chosen spin: ");
//    println!("{}", ising_spins.spin_config[0]);
//    println!("Lattice site: ");
//    println!("{}", points.lattice[0]);
    let mut site_energy = 0.0;

    for site_index in 0..ising_spins.spin_config.len() {
        let list_of_neighbors: Vec<Site> = Lattice::neighbor_list(points.lattice[site_index].x as i32, points.lattice[site_index].y as i32, n_x_size, n_y_size);

//    println!("List of neighbors: ");
//    for i in 0..4{
//        println!("{}", list_of_neighbors[i]);
//    }

//    println!("List of neighbors: ");
//    for i in 0..4{
//        println!("{}", Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, 4));
//    }

//    println!("List of neighbor spins: ");
//    for i in 0..4{
//        println!("{}", ising_spins.spin_config[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, 4) as usize]);
//    }



        for i in 0..4 {
            site_energy += -j * ising_spins.spin_config[site_index].dot(ising_spins.spin_config[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x_size) as usize]);
        }
//    println!("Site energy: ");
//    println!("{}", site_energy);
    }
    println!("{}", site_energy);
}