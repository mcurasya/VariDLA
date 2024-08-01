use obj::TexturedVertex;
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use std::collections::{hash_map, HashMap, HashSet};
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
        let unprocessed_vertices = &loaded_object.vertices;
        let mut graph = UnGraph::<Vertex, ()>::new_undirected();
        let mut processed_vertices = Vec::<Vertex>::new();
        let mut association = HashMap::new();
        for (index, vertex) in unprocessed_vertices.iter().enumerate() {
            let found_position = processed_vertices.iter().position(|&obj| {
                obj.x == vertex.position[0]
                    && obj.y == vertex.position[1]
                    && obj.z == vertex.position[2]
            });
            match found_position {
                Some(val) => {
                    association.insert(index, val);
                }
                None => {
                    processed_vertices.push(Vertex {
                        x: vertex.position[0],
                        y: vertex.position[1],
                        z: vertex.position[2],
                    });
                    association.insert(index, index);
                }
            }
        }
        for vert in &processed_vertices {
            graph.add_node(*vert);
        }
        let processed_indices: Vec<&usize> = loaded_object
            .indices
            .iter()
            .map(|&index| association.get(&(index as usize)).unwrap())
            .collect();
        let indices: Vec<_> = (0..loaded_object.indices.len())
            .into_iter()
            .step_by(3)
            .collect();
        for proc_index in &indices {
            graph.add_edge(NodeIndex::new(*processed_indices[*proc_index]), NodeIndex::new(*processed_indices[*proc_index+1]), ());
            graph.add_edge(NodeIndex::new(*processed_indices[*proc_index]), NodeIndex::new(*processed_indices[*proc_index+2]), ());
            graph.add_edge(NodeIndex::new(*processed_indices[*proc_index+1]), NodeIndex::new(*processed_indices[*proc_index+2]), ());
        }
        println!("{:#?}", loaded_object.indices);
        println!("{:#?}", &processed_vertices.len());
        println!("{:#?}", &processed_vertices);
        println!("{:#?}", &processed_indices);
        println!("{:#?}", &indices);
        println!(
            "{:#?}",
            loaded_object
                .indices
                .iter()
                .map(|&index| { association.get(&(index as usize)) })
                .step_by(3)
                .collect::<Vec<_>>()
        );

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
