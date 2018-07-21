use point::Point;

pub struct Lattice{
    lattice: Vec<Point>,
}

impl Lattice{
    pub fn generate_1d_lattice() -> Vec<Point>{
        let mut lat:Vec<Point> = Vec::new();
        for i in 0..10{
            lat.push(Point{x:i as f64, y:0.0, z:0.0});
        }
        lat
    }

    pub fn generate_square_lattice() -> Vec<Point>{
        let mut lat:Vec<Point> = Vec::new();
        for i in 0..10{
            for j in 0..10 {
                lat.push(Point { x: i as f64, y: j as f64, z: 0.0 });
            }
        }
        lat
    }
}