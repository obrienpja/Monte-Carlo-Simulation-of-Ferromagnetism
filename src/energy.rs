pub fn boltzmann_factor(energy:f64, temperature:f64) -> f64{
    let phase:f64 = -energy/temperature;
    phase.exp()
}