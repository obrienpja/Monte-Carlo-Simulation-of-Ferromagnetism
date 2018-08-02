use spin::{Spin, IsingSpin, SpinConfiguration, IsingSpinConfiguration};
use lattice::{Site, Lattice};

pub fn boltzmann_factor(energy:f64, temperature:f64) -> f64{
    let phase:f64 = -energy/temperature;
    phase.exp()
}

pub fn calculate_ising_energy(spin_config:&mut IsingSpinConfiguration) -> f64{
    let n_x_size = 4;
    let n_y_size = 4;
    let mut square_lattice = Lattice::generate_square_lattice(n_x_size,n_y_size);
    let neighbor_number = 2;

    let j_eng = 1.0;
    let mut energy = 0.0;

    for site_index in 0..spin_config.spin_config.len() {
        let site_spin: IsingSpin = spin_config.spin_config[site_index];
        let list_of_neighbors: Vec<Site> = Lattice::neighbor_list(square_lattice.lattice[site_index].x as i32, square_lattice.lattice[site_index].y as i32, n_x_size, n_y_size);

        for i in 0..neighbor_number {
            energy += (-j_eng) * site_spin.spin_value * spin_config.spin_config[Lattice::map_to_index(list_of_neighbors[i].x as i32, list_of_neighbors[i].y as i32, n_x_size) as usize].spin_value
        }
    }
    energy
}