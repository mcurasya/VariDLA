use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use std::collections::hash_set::Iter;
use std::io::Error;
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Clone)]
pub struct objtomanifold {
    pub graph: UnGraph<Vertex, ()>,
    // dividedGraph: UnGraph<Vertex, ()>,
}

impl objtomanifold {
    pub fn new(loaded_object: &obj::Obj<obj::TexturedVertex>) -> Result<objtomanifold, Error> {
        let positions = &loaded_object.vertices;
        let mut graph = UnGraph::<Vertex, ()>::new_undirected();
        let mut index = 1;
        for vertex in positions {
            let index = graph.add_node(Vertex {
                x: vertex.position[0],
                y: vertex.position[1],
                z: vertex.position[2],
            });
        }
        let indices: Vec<_> = (0..loaded_object.indices.len())
            .into_iter()
            .step_by(3)
            .collect();
        for index in indices {
            graph.update_edge(
                NodeIndex::new((loaded_object.indices[index]) as usize),
                NodeIndex::new((loaded_object.indices[index + 1]) as usize),
                (),
            );
            graph.update_edge(
                NodeIndex::new((loaded_object.indices[index]) as usize),
                NodeIndex::new((loaded_object.indices[index + 2]) as usize),
                (),
            );
            graph.update_edge(
                NodeIndex::new((loaded_object.indices[index + 1]) as usize),
                NodeIndex::new((loaded_object.indices[index + 2]) as usize),
                (),
            );
        }
        Ok(objtomanifold { graph })
    }

    fn distance_from_vertex(v: &Vertex, x: f32, y: f32, z: f32) -> f32 {
        ((x - v.x).powi(2) + (y - v.y).powi(2) + (z - v.z).powi(2)).sqrt()
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
