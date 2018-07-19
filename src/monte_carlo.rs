use spin::{Spin,SpinConfig};
use exchange_matrix::ExchangeMatrix;
extern crate rand;

use self::rand::Rng;

pub struct MonteCarlo {
//    spin_configuration: SpinConfig
}


impl MonteCarlo{
    pub fn run_monte_carlo(nitr:i32) -> SpinConfig{
        let mut test_spin_config:SpinConfig = SpinConfig::random_spin_config(10);
        let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);

        let mut rng = rand::thread_rng();
        for _i in 0..nitr{
            let new_spin:Spin = Spin::normalized_spin(&mut Spin::new());
            let n: usize = rng.gen_range(0, 10);
            let old_spin:Spin = test_spin_config.spin_config[n];
            let mut sum_eng:f64 = 0.0;

            for j in 0..10{
                sum_eng += &ferro_exchange.select_mat(n, j) * (new_spin - old_spin).dot(test_spin_config.spin_config[j])
            }
            sum_eng -= &ferro_exchange.select_mat(n, n) * (new_spin - old_spin).dot(test_spin_config.spin_config[n]);

            if sum_eng < 0.0 {
                test_spin_config.spin_config[n] = new_spin;
            }
        }
        test_spin_config
    }
}