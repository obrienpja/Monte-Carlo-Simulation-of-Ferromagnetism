use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value, Error};
use SpinConfig;
use serde_json;

pub fn write_plot(spin_configs: Vec<SpinConfig>) {
    let mut file = File::create("figs/data.json").unwrap();
    for spin_config in &spin_configs {
        write!(file, "{}", serde_json::to_string(spin_config).unwrap());
    }
}