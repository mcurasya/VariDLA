use blue_engine::header::Engine;
use blue_engine::primitive_shapes::uv_sphere;
use classes::dlamanif::Particle;
use classes::objtomanifold::objtomanifold;
use gl::GREATER;
use obj::{self};
use petgraph::graph::NodeIndex;
use petgraph::visit::NodeRef;
use rfd;
mod classes;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
fn main() {
    let path = std::env::current_dir().unwrap();
    let filepath = rfd::FileDialog::new()
        .set_directory(&path)
        .pick_file()
        .unwrap();
    println!("{:?}", filepath);
    let buffer = BufReader::new(File::open(filepath).unwrap());
    let model: obj::Obj<obj::TexturedVertex> =
        obj::load_obj(buffer).expect("error happened while loading model");
    let mut engine = Engine::new().expect("win");

    let indices = model.indices[..].to_vec();
    engine
        .objects
        .new_object(
            "head",
            model
                .vertices
                .iter()
                .map(|vertex| {
                    let vertex = vertex;
                    let pos = vertex.position;
                    let norm = vertex.normal;
                    let uv = [vertex.texture[0], vertex.texture[1]];
                    blue_engine::Vertex {
                        position: pos,
                        uv,
                        normal: norm,
                    }
                })
                .collect(),
            indices,
            blue_engine::ObjectSettings {
                ..Default::default()
            },
            &mut engine.renderer,
        )
        .unwrap();
    let radius = 40f32;
    let start = std::time::SystemTime::now();
    let object = objtomanifold::new(&model).unwrap();
    let not_moving = object.graph.node_weight(NodeIndex::new(4)).unwrap();
    uv_sphere(
        "initial",
        (5, 5, 1f32),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();
    engine.objects.get_mut("initial").unwrap().set_position(
        not_moving.x,
        not_moving.y,
        not_moving.z,
    );
    engine
        .objects
        .get_mut("initial")
        .unwrap()
        .set_color(0.0, 0.0, 1.0, 1.0)
        .unwrap();
    let graph = Box::new(&object.graph);

    let starting_position = *graph.node_weight(NodeIndex::new(1000)).unwrap();
    let mut starting_particle = Particle::new_on_lattice(
        starting_position.x,
        starting_position.y,
        starting_position.z,
        &1000,
        "particle1",
    );
    uv_sphere(
        "starter",
        (5, 5, 1f32),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();
    engine.objects.get_mut("starter").unwrap().set_position(
        starting_position.x,
        starting_position.y,
        starting_position.z,
    );
    engine
        .objects
        .get_mut("starter")
        .unwrap()
        .set_color(1.0, 0.0, 0.0, 1.0)
        .unwrap();
    uv_sphere(
        starting_particle.name.clone(),
        (5, 5, 2f32),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();
    engine
        .objects
        .get_mut(&starting_particle.name.clone())
        .unwrap()
        .set_position(
            starting_position.x,
            starting_position.y,
            starting_position.z,
        );
    engine
        .objects
        .get_mut(&starting_particle.name.clone())
        .unwrap()
        .set_color(1.0, 1.0, 0.0, 1.0)
        .unwrap();

    engine
        .update_loop(move |_, _, objStorage, _, camera, _| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            let mut neighbours = object.graph.neighbors_undirected(NodeIndex::new(
                **starting_particle.vertex_index.as_ref().unwrap() as usize,
            ));
            let chosen = neighbours.next().unwrap();
            let new_vert = object.graph.node_weight(chosen).unwrap();
            // println!("{:#?}", neighbours);
            starting_particle.vertex_index = Some(Rc::new(chosen.index() as u32));
            starting_particle.x = new_vert.x;
            starting_particle.y = new_vert.y;
            starting_particle.z = new_vert.z;
            objStorage
                .get_mut(&starting_particle.name.clone())
                .unwrap()
                .set_position(
                    starting_position.x,
                    starting_position.y,
                    starting_position.z,
                );
            camera
                .set_position(camx, camy, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}
