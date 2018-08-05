use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value, Error};
use spin::SpinConfiguration;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct CartesianPoint {
    x: f64,
    y: f64,
    z: Option<f64>
}

trait Plot {
    fn get_points() -> Vec<CartesianPoint>;
}


pub fn write_plot(spin_configs: Vec<SpinConfiguration>) {
    let mut file = File::create("figs/data.json").unwrap();
    write!(file, "{}", serde_json::to_string(&spin_configs).unwrap());
}