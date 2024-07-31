use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use std::io::Error;
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Clone)]
pub struct ObjToManifold {
    pub graph: UnGraph<Vertex, ()>,
    // dividedGraph: UnGraph<Vertex, ()>,
}

impl ObjToManifold {
    pub fn new(loaded_object: &obj::Obj<obj::TexturedVertex>) -> Result<ObjToManifold, Error> {
        let positions = &loaded_object.vertices;
        println!("{:#?}",positions.len());
        println!("{:#?}",positions);
        let mut graph = UnGraph::<Vertex, ()>::new_undirected();
        for vertex in positions {
            graph.add_node(Vertex {
                x: vertex.position[0],
                y: vertex.position[1],
                z: vertex.position[2],
            });
        }
        let indices: Vec<_> = (0..loaded_object.indices.len())
            .into_iter()
            .step_by(3)
            .collect();
        println!("{:#?}", loaded_object.indices);
        println!("{:#?}", indices);
        println!("{:#?}", loaded_object.indices.iter().step_by(3).collect::<Vec<_>>());
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
            graph.update_edge(
                NodeIndex::new((loaded_object.indices[index + 1]) as usize),
                NodeIndex::new((loaded_object.indices[index]) as usize),
                (),
            );
            graph.update_edge(
                NodeIndex::new((loaded_object.indices[index + 2]) as usize),
                NodeIndex::new((loaded_object.indices[index]) as usize),
                (),
            );
            graph.update_edge(
                NodeIndex::new((loaded_object.indices[index + 2]) as usize),
                NodeIndex::new((loaded_object.indices[index + 1]) as usize),
                (),
            );
        }
        Ok(ObjToManifold { graph })
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
