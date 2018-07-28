use point::Point;
use std::prelude::v1::Vec;

pub struct Lattice{
    pub lattice: Vec<Point>,
    pub primitive_vectors: Vec<Point>,
    pub basis_vectors: Vec<Point>
}

impl Lattice {
    fn generate_1d_lattice() -> Vec<Point> {
        let mut lat: Vec<Point> = Vec::new();
        for i in 0..10 {
            lat.push(Point { x: i as f64, y: 0.0, z: 0.0 });
        }
        lat
    }
}

impl Lattice{
    pub fn generate_square_lattice() -> Lattice{
        let mut lat:Vec<Point> = Vec::new();
        for j in 0..10{
            for i in 0..10 {
                lat.push(Point { x: i as f64, y: j as f64, z: 0.0 });
            }
        }
        Lattice{lattice:lat, primitive_vectors:Vec::new(), basis_vectors:Vec::new()}
    }
}

impl Lattice{
    fn mapping(m_x:i32, m_y:i32, n_x:i32) -> i32{
        m_y*n_x + m_x
    }
}

impl Lattice{
    fn neighbor_list(m_x:i32, m_y:i32, n_x:i32) -> Vec<Point>{
        let mut neighbors:Vec<Point> = Vec::new();
        neighbors.push(Point{x:(m_x - 1) as f64, y:m_y as f64, z:0.0});
        neighbors.push(Point{x:(m_x + 1) as f64, y:m_y as f64, z:0.0});
        neighbors.push(Point{x:m_x as f64, y:(m_y - 1) as f64, z:0.0});
        neighbors.push(Point{x:m_x as f64, y:(m_y + 1) as f64, z:0.0});
        neighbors
    }
}