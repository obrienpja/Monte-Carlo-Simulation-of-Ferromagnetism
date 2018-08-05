extern crate rulinalg;
extern crate rand;

use self::rand::Rng;
use std::f64;
use std::fmt;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Neg;
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
pub struct SpinConfiguration{
    pub spin_configuration: Vec<Spin>,
}

impl SpinConfiguration{
    pub fn random_spin_configuration(n:i32) -> SpinConfiguration{
        let mut spin_configuration_temp:Vec<Spin> = Vec::new();

        for _i in 0..n{
            spin_configuration_temp.push(Spin::normalized_spin(&mut Spin::new()));
        }
        SpinConfiguration{spin_configuration: spin_configuration_temp}
    }
}

impl SpinConfiguration{
    pub fn random_ising_spin_configuration(n:i32) -> SpinConfiguration{
        let mut spin_configuration_temp:Vec<Spin> = Vec::new();

        for _i in 0..n{
            spin_configuration_temp.push(Spin::create_random_ising_spin());
        }
        SpinConfiguration{spin_configuration: spin_configuration_temp}
    }
}

impl SpinConfiguration{
    pub fn print_spin_configuration(&mut self) ->(){
        for i in 0..self.spin_configuration.len(){
            println!("{}", self.spin_configuration[i]);
        }
    }

    pub fn plot_spin_configuration(&mut self, size_of_config:usize) ->(){
        for i in 0..size_of_config{
//            plot::plot(self.spin_configuration[i].x, self.spin_configuration[i].y, self.spin_configuration[i].z, i);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Copy, Clone)]
pub struct IsingSpin{
    pub spin_value:f64
}

impl IsingSpin{
    pub fn create_random_ising_spin() -> IsingSpin{
        let mut rng = rand::thread_rng();
        IsingSpin{spin_value:(rng.gen_range(0, 2)*2 - 1) as f64}
    }
}

//impl Clone for Vec<IsingSpin> {
//    fn clone(&self) -> Vec { *self }
//}

//#[derive(Copy, Clone)]
pub struct IsingSpinConfiguration{
    pub spin_configuration: Vec<IsingSpin>
}

impl IsingSpinConfiguration{
    pub fn random_ising_spin_configuration(n:i32) -> IsingSpinConfiguration{
        let mut ising_spin_configuration:Vec<IsingSpin> = Vec::new();

        for _i in 0..n{
            ising_spin_configuration.push(IsingSpin::create_random_ising_spin());
        }
        IsingSpinConfiguration{spin_configuration: ising_spin_configuration}
    }
}

impl Sub<IsingSpin> for IsingSpin {
    type Output = Self;
    fn sub(self, other: IsingSpin) -> Self {
        IsingSpin{spin_value: self.spin_value - other.spin_value}
    }
}

impl Mul<IsingSpin> for IsingSpin {
    type Output = Self;
    fn mul(self, other: IsingSpin) -> Self {
        IsingSpin{spin_value: self.spin_value * other.spin_value}
    }
}

impl Mul<f64> for IsingSpin {
    type Output = Self;
    fn mul(self, constant:f64) -> Self {
        IsingSpin{spin_value: self.spin_value * constant}
    }
}

impl Neg for IsingSpin {
    type Output = Self;
    fn neg(self) -> Self {
        IsingSpin {spin_value: -self.spin_value}
    }
}

impl fmt::Display for IsingSpin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.spin_value)
    }
}

impl IsingSpinConfiguration{
    pub fn print_spin_configuration(&mut self) ->(){
        for i in 0..self.spin_configuration.len(){
            println!("{}", self.spin_configuration[i]);
        }
    }
}