extern crate rand;
use rand::{thread_rng, Rng};

mod spin;
use spin::Spin;

mod exchange_matrix;
use exchange_matrix::ExchangeMatrix;

fn main()
{
    let mut spin_config:Vec<Spin> = Vec::new();

    for _i in 0..10{
        spin_config.push(Spin::normalized_spin(&mut Spin::new()));
    }

    let mut ferromagnet:Vec<Spin> = Vec::new();

    for _i in 0..10{
        ferromagnet.push(Spin{x:0.0, y:0.0, z:1.0});
    }


    let mut energy:f64 = 0.0;

    let mut ferro_exchange: ExchangeMatrix = ExchangeMatrix::ferromagnetic_exchange(10);

    let mut spin_config:Vec<Spin> = Vec::new();

    for _i in 0..10{
        spin_config.push(Spin::normalized_spin(&mut Spin::new()));
    }

    let mut rng = thread_rng();
    for _i in 0..1000000{
        let new_spin:Spin = Spin::normalized_spin(&mut Spin::new());
        let n: usize = rng.gen_range(0, 10);
        let old_spin:Spin = spin_config[n];
        let mut sum_eng:f64 = 0.0;

        for j in 0..10{
            sum_eng += &ferro_exchange.select_mat(n, j) * (new_spin - old_spin).dot(spin_config[j])
        }
        sum_eng -= &ferro_exchange.select_mat(n, n) * (new_spin - old_spin).dot(spin_config[n]);

        if sum_eng < 0.0 {
            spin_config[n] = new_spin;
        }
    }


    for i in 0..10{
        for j in 0..10{
            energy += (spin_config[i]).dot(spin_config[j]) * &ferro_exchange.select_mat(i, j)
        }
    }

    println!("{}", energy);
}