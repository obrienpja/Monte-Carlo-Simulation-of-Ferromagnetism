use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value, Error};
use spin::SpinConfiguration;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct CartesianPoint {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chart {
    pub data: Vec<CartesianPoint>,
    pub plot_type: String,
    pub plot_name: String
}

trait Plot {
    fn get_plot() -> Chart;
}


pub fn write_plot(chart: Chart) {
    let mut file = File::create("figs/data.json").unwrap();
    write!(file, "{}", serde_json::to_string(&chart).unwrap());
}