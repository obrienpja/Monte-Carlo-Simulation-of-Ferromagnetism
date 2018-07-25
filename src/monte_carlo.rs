use spin::{Spin,SpinConfig};
use exchange_matrix::ExchangeMatrix;
extern crate rand;

use self::rand::Rng;

pub struct MonteCarlo {
//    spin_configuration: SpinConfig
}


impl MonteCarlo{
    pub fn run_monte_carlo(nitr:i32, ex_mat:&mut ExchangeMatrix) -> SpinConfig{
        let mut spin_config:SpinConfig = SpinConfig::random_spin_config(10);

        let mut rng = rand::thread_rng();
        for _i in 0..nitr{
            let new_spin:Spin = Spin::normalized_spin(&mut Spin::new());
            let n: usize = rng.gen_range(0, 10);
            let old_spin:Spin = spin_config.spin_config[n];
            let mut sum_eng:f64 = 0.0;

            for j in 0..10{
                sum_eng += &ex_mat.select_mat(n, j) * (new_spin - old_spin).dot(spin_config.spin_config[j])
            }
            sum_eng -= &ex_mat.select_mat(n, n) * (new_spin - old_spin).dot(spin_config.spin_config[n]);

            if sum_eng < 0.0 {
                spin_config.plot_spin_config(10);
                spin_config.spin_config[n] = new_spin;

            }
        }
        spin_config
    }
}