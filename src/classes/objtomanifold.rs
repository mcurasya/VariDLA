use std::{io::Error, rc::{Rc}};

use blue_engine::winit::error::NotSupportedError;
use obj::{self, raw::object};
use petgraph::prelude::*;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};


struct Vertex (f32, f32, f32);

pub struct objtomanifold {
    graph: GraphMap<Vertex, (), Undirected>
}


impl objtomanifold {
    pub fn new(loaded_object:&obj::Obj<obj::TexturedVertex>) -> Result<objtomanifold, Error> {
        let positions = &loaded_object.vertices;
        let mut graph = UnGraph::<objtomanifold, ()>::new_undirected();
        
        Err(Error::new(std::io::ErrorKind::NotFound, "not finished realizing"))
    }

    fn distance_from_vertex(v: &Vertex, x: f32, y: f32, z: f32) -> f32 {
        ((x - v.0).powi(2) + (y - v.1).powi(2) + (z - v.2).powi(2)).sqrt()
    }
    // pub fn get_n_nearest_points(
    //     &self,
    //     n: usize,
    //     x: f32,
    //     y: f32,
    //     z: f32,
    // ) -> Result<Vec<>, Error> {
    //     todo!();
    // }
}
