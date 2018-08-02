use spin::{Spin,SpinConfiguration, IsingSpin, IsingSpinConfiguration};
use exchange_matrix::ExchangeMatrix;
extern crate rand;

use self::rand::Rng;
use lattice::{Site, Lattice};
use energy::{calculate_ising_energy, boltzmann_factor};

pub struct MonteCarlo {
//    spin_configuration: SpinConfig
}


impl MonteCarlo {
    pub fn run_monte_carlo(nitr: i32, ex_mat: &mut ExchangeMatrix) -> SpinConfiguration {
        let mut spin_config: SpinConfiguration = SpinConfiguration::random_spin_config(10);

        let mut rng = rand::thread_rng();
        for _i in 0..nitr {
            let new_spin: Spin = Spin::normalized_spin(&mut Spin::new());
            let n: usize = rng.gen_range(0, 10);
            let old_spin: Spin = spin_config.spin_config[n];
            let mut sum_eng: f64 = 0.0;

            for j in 0..10 {
                sum_eng += &ex_mat.select_mat(n, j) * (new_spin - old_spin).dot(spin_config.spin_config[j])
            }
            sum_eng -= &ex_mat.select_mat(n, n) * (new_spin - old_spin).dot(spin_config.spin_config[n]);

            if sum_eng < 0.0 {
                spin_config.spin_config[n] = new_spin;
            }
        }
        spin_config
    }

    pub fn run_ising_monte_carlo(nitr: i32, mc_temperature:f64) -> IsingSpinConfiguration {
        let j_eng = 1.0;
        let n_x_size = 4;
        let n_y_size = 4;
        let neighbor_number = 4;
        let mut site_energy = 0.0;
        let mut square_lattice = Lattice::generate_square_lattice(n_x_size,n_y_size);
        let mut energy_list:Vec<f64> = Vec::new();

        println!("mc_temperature: {}", mc_temperature);

        let mut spin_config: IsingSpinConfiguration = IsingSpinConfiguration::random_ising_spin_config(n_x_size*n_y_size);
        println!("initial ising energy: {}", calculate_ising_energy(&mut spin_config));
        energy_list.push(calculate_ising_energy(&mut spin_config));

        println!("The initial Ising spin configuration is: ");
        spin_config.print_spin_config();

        let mut rng = rand::thread_rng();
        let mut accepted = 0;

        for i in 0..nitr {
            println!("Iteration: {}", i);
            let site_index: usize = rng.gen_range(0, (n_x_size*n_y_size) as usize);
            println!("The site index is: {}", site_index);
            let old_spin: IsingSpin = spin_config.spin_config[site_index];
            println!("The old spin is: {}", old_spin);
            let new_spin: IsingSpin = -old_spin;
            println!("The new spin is: {}", new_spin);
            let mut energy_change: f64 = 0.0;

            println!("{}", new_spin-old_spin);

            let list_of_neighbors: Vec<Site> = Lattice::neighbor_list_all(square_lattice.lattice[site_index].x as i32, square_lattice.lattice[site_index].y as i32, n_x_size, n_y_size);

            for i in 0..neighbor_number {
                energy_change += (-j_eng) * (new_spin - old_spin).spin_value * spin_config.spin_config[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x_size) as usize].spin_value
            }
            println!("energy change: {}", energy_change);

            let random_number = rng.gen::<f64>();
            println!("random number: {}", random_number);
            println!("boltzmann_factor: {}", boltzmann_factor(energy_change, mc_temperature));
            if random_number < boltzmann_factor(energy_change, mc_temperature) {
                spin_config.spin_config[site_index] = new_spin;
                println!("accepted, energy_change: {}", energy_change);
                spin_config.print_spin_config();
                energy_list.push(energy_change);
                accepted += 1;
            }

        }
        println!("{:?}", energy_list);

        spin_config
    }
}