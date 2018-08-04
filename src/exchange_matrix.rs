extern crate rulinalg;
extern crate num_complex;
use self::rulinalg::matrix::{Matrix, BaseMatrix};
use std::f64;
use spin::SpinConfiguration;
use point::Point;
use self::num_complex::Complex;
use self::f64::consts;

pub struct ExchangeMatrix{
    pub exchange_matrix: Matrix<f64>
}

impl ExchangeMatrix{
    pub fn ferromagnetic_exchange(n:usize) -> ExchangeMatrix{
        ExchangeMatrix{exchange_matrix : -Matrix::new(n,n, (1..(n.pow(2)+1)).map(|_v| 1 as f64).collect::<Vec<f64>>())}
    }

    pub fn antiferromagnetic_exchange(n:usize) -> ExchangeMatrix{
        ExchangeMatrix{exchange_matrix : Matrix::new(n,n, (1..(n.pow(2)+1)).map(|_v| 1 as f64).collect::<Vec<f64>>())}
    }

    pub fn select_mat(&mut self, r:usize, c:usize) -> f64{
        self.exchange_matrix.select(&[r], &[c]).data()[0]
    }

    pub fn calculate_energy(&mut self, spin_configuration:SpinConfiguration) -> f64{
        let mut energy:f64 = 0.0;

        for i in 0..10{
            for j in 0..10{
                energy += (spin_configuration.spin_configuration[i]).dot(spin_configuration.spin_configuration[j]) * self.select_mat(i, j)
            }
        }
        energy
    }
}


impl ExchangeMatrix{
    pub fn phase_factor(pt:Point, pt2:Point) -> Complex<f64>{
        let phase:Complex<f64> = self::f64::consts::PI * Complex::i() * pt.dot(pt2);
        phase.exp()
    }
}