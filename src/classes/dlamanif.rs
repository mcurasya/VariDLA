
use petgraph::adj::NodeIndex;


#[derive(Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vertex_index: Option<Box<NodeIndex>>,
    pub name: String,
}

impl Particle {
    pub fn new_on_lattice(x: f32, y: f32, z: f32, vertex: &NodeIndex, name: &str) -> Particle {
        Particle {
            x,
            y,
            z,
            vertex_index: Some(Box::new(*vertex)),
            name: String::from(name),
        }
    }
}
