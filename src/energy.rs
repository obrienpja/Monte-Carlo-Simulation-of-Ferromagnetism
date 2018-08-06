use spin::{Spin, IsingSpin, SpinConfiguration, IsingSpinConfiguration};
use lattice::{Site, Lattice};

pub fn boltzmann_factor(energy:f64, temperature:f64) -> f64{
    let phase:f64 = -energy/temperature;
    phase.exp()
}

pub fn calculate_ising_energy(n_x:i32, n_y:i32, spin_configuration:&mut IsingSpinConfiguration) -> f64{
    let mut square_lattice = Lattice::generate_square_lattice(n_x,n_y);
    let neighbor_number = 2;

    let j_eng = 1.0;
    let mut energy = 0.0;

    for site_index in 0..spin_configuration.spin_configuration.len() {
        let site_spin: IsingSpin = spin_configuration.spin_configuration[site_index];
        let list_of_neighbors: Vec<Site> = Lattice::neighbor_list(square_lattice.lattice[site_index].x as i32, square_lattice.lattice[site_index].y as i32, n_x, n_y);

        for i in 0..neighbor_number {
            energy += (-j_eng) * site_spin.spin_value * spin_configuration.spin_configuration[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x) as usize].spin_value
        }
    }
    energy
}

pub fn running_total(vector:Vec<f64>) -> Vec<f64>{
    let mut total:Vec<f64> = Vec::new();
    for i in 0..vector.len() {
        let mut sum_of_engs = 0.0;
        for j in 0..i+1 {
            sum_of_engs += vector[j];
        }
        total.push(sum_of_engs);
    }
    total
}

pub fn running_total_squared(vector:Vec<f64>) -> Vec<f64>{
    let mut total:Vec<f64> = Vec::new();
    for i in 0..vector.len() {
        let mut sum_of_engs = 0.0;
        for j in 0..i+1 {
            sum_of_engs += vector[j];
        }
        total.push(sum_of_engs.powf(2.0));
    }
    total
}

pub fn average(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}