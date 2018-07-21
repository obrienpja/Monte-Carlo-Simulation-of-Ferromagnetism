use std::fmt;

pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

//impl Point{
//    pub fn print_spin_config(&mut self, size_of_config:usize) ->(){
//        for i in 0..size_of_config{
//            println!("{}", self.spin_config[i]);
//        }
//    }
//}