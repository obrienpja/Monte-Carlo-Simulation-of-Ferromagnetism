use point::Point;
use std::prelude::v1::Vec;
use std::fmt;

pub struct Site{
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub struct Lattice{
    pub lattice: Vec<Site>,
    pub primitive_vectors: Vec<Point>,
    pub basis_vectors: Vec<Point>
}

impl Lattice {
    pub fn generate_1d_lattice() -> Vec<Site> {
        let mut lat: Vec<Site> = Vec::new();
        for i in 0..10 {
            lat.push(Site {x: i as f64, y: 0.0, z: 0.0});
        }
        lat
    }
}

impl Lattice{
    pub fn generate_square_lattice(n_x:i32, n_y:i32) -> Lattice{
        let mut lat:Vec<Site> = Vec::new();
        for j in 0..n_y{
            for i in 0..n_x {
                lat.push(Site { x: i as f64, y: j as f64, z: 0.0 });
            }
        }
        Lattice{lattice:lat, primitive_vectors:Vec::new(), basis_vectors:Vec::new()}
    }
}

impl Lattice{
    pub fn map_to_index(m_x:i32, m_y:i32, n_x:i32) -> i32{
        m_y*n_x + m_x
    }
}

impl Lattice{
    pub fn map_to_site(index:i32, n_x:i32) -> Site{
        Site{x: (index - n_x*(index/n_x)) as f64, y:(index/n_x) as f64, z:0.0}
    }
}

impl Lattice{
    pub fn neighbor_list(m_x:i32, m_y:i32, n_x:i32, n_y:i32) -> Vec<Site>{
        let mut neighbors:Vec<Site> = Vec::new();
        neighbors.push(Site{x:((m_x + 1 + n_x)%n_x) as f64, y:m_y as f64, z:0.0});
        neighbors.push(Site{x:m_x as f64, y:((m_y + 1 + n_y)%n_y) as f64, z:0.0});
        neighbors
    }
}

impl Lattice{
    pub fn neighbor_list_all(m_x:i32, m_y:i32, n_x:i32, n_y:i32) -> Vec<Site>{
        let mut neighbors:Vec<Site> = Vec::new();
        neighbors.push(Site{x:((m_x + 1 + n_x)%n_x) as f64, y:m_y as f64, z:0.0});
        neighbors.push(Site{x:((m_x - 1 + n_x)%n_x) as f64, y:m_y as f64, z:0.0});
        neighbors.push(Site{x:m_x as f64, y:((m_y + 1 + n_y)%n_y) as f64, z:0.0});
        neighbors.push(Site{x:m_x as f64, y:((m_y - 1 + n_y)%n_y) as f64, z:0.0});
        neighbors
    }
}

impl Lattice{
    pub fn print_lattice(&mut self) ->(){
        for i in 0..self.lattice.len(){
            println!("{}", self.lattice[i]);
        }
    }
}
