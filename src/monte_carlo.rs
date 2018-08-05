extern crate rand;

use spin::{Spin, SpinConfiguration, IsingSpin, IsingSpinConfiguration};
use exchange_matrix::ExchangeMatrix;
use self::rand::Rng;
use lattice::{Site, Lattice};
use energy::{calculate_ising_energy, boltzmann_factor, running_total, average};
//use self::MonteCarlo;

pub struct MonteCarlo {
//    spin_configurationuration: SpinConfig
}



    pub fn run_monte_carlo(nitr: i32, ex_mat: &mut ExchangeMatrix) -> SpinConfiguration {
        let mut spin_configuration: SpinConfiguration = SpinConfiguration::random_spin_configuration(10);

        let mut random_number_generator = rand::thread_rng();
        for _i in 0..nitr {
            let new_spin: Spin = Spin::normalized_spin(&mut Spin::new());
            let random_site_index: usize = random_number_generator.gen_range(0, 10);
            let old_spin: Spin = spin_configuration.spin_configuration[random_site_index];
            let mut sum_eng: f64 = 0.0;

            for j in 0..10 {
                sum_eng += &ex_mat.select_mat(random_site_index, j) * (new_spin - old_spin).dot(spin_configuration.spin_configuration[j])
            }
            sum_eng -= &ex_mat.select_mat(random_site_index, random_site_index) * (new_spin - old_spin).dot(spin_configuration.spin_configuration[random_site_index]);

            if sum_eng < 0.0 {
                spin_configuration.spin_configuration[random_site_index] = new_spin;
            }
        }
        spin_configuration
    }

//    pub fn monte_carlo() -> () {
//        for i in 0..number_of_settling_iterations {
//            let random_site_index: usize = random_number_generator.gen_range(0, (n_x_size * n_y) as usize);
//            let old_spin: IsingSpin = spin_configuration.spin_configuration[random_site_index];
//            let new_spin: IsingSpin = -old_spin;
//            let mut energy_change: f64 = 0.0;
//
//            let list_of_neighbors: Vec<Site> = Lattice::neighbor_list_all(square_lattice.lattice[random_site_index].x as i32, square_lattice.lattice[random_site_index].y as i32, n_x_size, n_y);
//
//            for i in 0..neighbor_number {
//                energy_change += (-j_eng) * (new_spin - old_spin).spin_value * spin_configuration.spin_configuration[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x_size) as usize].spin_value
//            }
//
//            let random_number = random_number_generator.gen::<f64>();
//
//            if random_number < boltzmann_factor(energy_change, mc_temperature) {
//                spin_configuration.spin_configuration[random_site_index] = new_spin;
//                spin_configuration.print_spin_configuration();
//                energy_list.push(energy_change);
//                accepted += 1;
//            }
//        }
//    }

    pub fn run_ising_monte_carlo(n_x:i32, n_y:i32, number_of_settling_iterations: i32, number_of_data_collection_iterations: i32, mc_temperature: f64) -> f64 {

        let j_eng = 1.0;
        let neighbor_number = 4;
        let mut site_energy = 0.0;
        let mut square_lattice = Lattice::generate_square_lattice(n_x, n_y);
        let mut energy_list: Vec<f64> = Vec::new();

        let mut spin_configuration: IsingSpinConfiguration = IsingSpinConfiguration::random_ising_spin_configuration(n_x * n_y);

//        spin_configuration.print_spin_configuration();

        let mut random_number_generator = rand::thread_rng();
        let mut accepted = 0;


        for i in 0..number_of_settling_iterations {
            let random_site_index: usize = random_number_generator.gen_range(0, (n_x * n_y) as usize);
            let old_spin: IsingSpin = spin_configuration.spin_configuration[random_site_index];
            let new_spin: IsingSpin = -old_spin;
            let mut energy_change: f64 = 0.0;

            let list_of_neighbors: Vec<Site> = Lattice::neighbor_list_all(square_lattice.lattice[random_site_index].x as i32, square_lattice.lattice[random_site_index].y as i32, n_x, n_y);

            for i in 0..neighbor_number {
                energy_change += (-j_eng) * (new_spin - old_spin).spin_value * spin_configuration.spin_configuration[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x) as usize].spin_value
            }

            let random_number = random_number_generator.gen::<f64>();

            if random_number < boltzmann_factor(energy_change, mc_temperature) {
                spin_configuration.spin_configuration[random_site_index] = new_spin;
//                spin_configuration.print_spin_configuration();
                accepted += 1;
            }
        }

        energy_list.push(calculate_ising_energy(n_x, n_y, &mut spin_configuration));

        for i in 0..number_of_data_collection_iterations {
            let random_site_index: usize = random_number_generator.gen_range(0, (n_x * n_y) as usize);
            let old_spin: IsingSpin = spin_configuration.spin_configuration[random_site_index];
            let new_spin: IsingSpin = -old_spin;
            let mut energy_change: f64 = 0.0;

            let list_of_neighbors: Vec<Site> = Lattice::neighbor_list_all(square_lattice.lattice[random_site_index].x as i32, square_lattice.lattice[random_site_index].y as i32, n_x, n_y);

            for i in 0..neighbor_number {
                energy_change += (-j_eng) * (new_spin - old_spin).spin_value * spin_configuration.spin_configuration[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x) as usize].spin_value
            }

            let random_number = random_number_generator.gen::<f64>();

            if random_number < boltzmann_factor(energy_change, mc_temperature) {
                spin_configuration.spin_configuration[random_site_index] = new_spin;
//                spin_configuration.print_spin_configuration();
                energy_list.push(energy_change);
                accepted += 1;
            }
        }
        let mut energies = running_total(energy_list);
//        println!("The average energy is {}", average(&energies)/((n_x*n_y) as f64));

        average(&energies)/((n_x*n_y) as f64)
    }
impl MonteCarlo {
    pub fn metrolpolis_algorithm(n_x:i32, n_y:i32) -> Vec<f64>{
        let mut average_energy_vs_temperature = Vec::new();
        let ising_configuration = IsingSpinConfiguration::random_ising_spin_configuration(n_x * n_y);
        for temperature in 0..81{
            println!("{}", temperature);
            average_energy_vs_temperature.push(run_ising_monte_carlo(n_x, n_y, 20000, 20000, 0.05 * temperature as f64 + 1.0));
        }
        average_energy_vs_temperature
    }
}