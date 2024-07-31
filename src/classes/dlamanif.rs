use std::cell::RefCell;
use std::rc::Rc;

use petgraph::adj::NodeIndex;
use petgraph::graph::Node;

use crate as classes;

use super::objtomanifold::ObjToManifold;
use super::objtomanifold::Vertex;
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
