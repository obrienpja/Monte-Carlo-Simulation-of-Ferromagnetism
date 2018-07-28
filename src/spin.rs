extern crate rulinalg;
extern crate rand;

use self::rand::Rng;
use std::f64;
use std::fmt;
use std::ops::Div;
use std::ops::Sub;
use plot;

#[derive(Serialize, Deserialize, Debug)]
#[derive(Copy, Clone)]
pub struct Spin {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Spin {
    pub fn new() -> Spin {
        let mut rng = rand::thread_rng();
        Spin { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>() }
    }
    pub fn normalized_spin(&mut self) -> Spin
    {
        let normalization = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        self.x /= normalization;
        self.y /= normalization;
        self.z /= normalization;
        *self
    }
    pub fn dot(self, second_spin: Spin) -> f64 {
        self.x * second_spin.x + self.y * second_spin.y + self.z * second_spin.z
    }
}

impl Div<f64> for Spin {
    type Output = Self;
    fn div(self, denom: f64) -> Self {
        Spin { x: self.x / denom, y: self.y / denom, z: self.z / denom }
    }
}

impl Sub<Spin> for Spin {
    type Output = Self;
    fn sub(self, other: Spin) -> Self {
        Spin { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl fmt::Display for Spin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Spin{
    pub fn create_random_ising_spin() -> Spin{
        let mut rng = rand::thread_rng();
        Spin{x: 0.0, y: 0.0, z:(rng.gen_range(0, 2)*2 - 1) as f64}
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SpinConfig{
    pub spin_config: Vec<Spin>,
}

impl SpinConfig{
    pub fn random_spin_config(n:i32) -> SpinConfig{
        let mut spin_config_temp:Vec<Spin> = Vec::new();

        for _i in 0..n{
            spin_config_temp.push(Spin::normalized_spin(&mut Spin::new()));
        }
        SpinConfig{spin_config: spin_config_temp}
    }
}

impl SpinConfig{
    pub fn random_ising_spin_config(n:i32) -> SpinConfig{
        let mut spin_config_temp:Vec<Spin> = Vec::new();

        for _i in 0..n{
            spin_config_temp.push(Spin::create_random_ising_spin());
        }
        SpinConfig{spin_config: spin_config_temp}
    }
}

impl SpinConfig{
    pub fn print_spin_config(&mut self) ->(){
        for i in 0..self.spin_config.len(){
            println!("{}", self.spin_config[i]);
        }
    }

    pub fn plot_spin_config(&mut self, size_of_config:usize) ->(){
        for i in 0..size_of_config{
//            plot::plot(self.spin_config[i].x, self.spin_config[i].y, self.spin_config[i].z, i);
        }
    }
}

pub struct IsingSpin{
    val:i32
}

