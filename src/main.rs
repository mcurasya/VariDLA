use blue_engine::header::Engine;
use blue_engine::primitive_shapes::uv_sphere;
use classes::dlamanif::Particle;
use classes::objtomanifold::ObjToManifold;
use obj::{self};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use rfd;
use std::fs::File;
use std::io::BufReader;
mod classes;
fn main() {
    let path = std::env::current_dir().unwrap();
    let filepath = rfd::FileDialog::new()
        .set_directory(&path)
        .pick_file()
        .unwrap();
    let buffer = BufReader::new(File::open(filepath).unwrap());
    let model: obj::Obj<obj::TexturedVertex> =
        obj::load_obj(buffer).expect("error happened while loading model");
    let mut engine = Engine::new().expect("win");
    println!("{:#?}", model.vertices.len());
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
    let start = std::time::SystemTime::now();
    let object = ObjToManifold::new(&model).unwrap();
    let not_moving = object.graph.node_weight(NodeIndex::new(4)).unwrap();
    uv_sphere(
        "initial",
        (5, 5, 0.2),
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
    let start_index = 1;
    let starting_position = *graph.node_weight(NodeIndex::new(start_index)).unwrap();
    let mut starting_particle = Particle::new_on_lattice(
        starting_position.x,
        starting_position.y,
        starting_position.z,
        &(start_index as u32),
        "particle1",
    );
    uv_sphere(
        "starter",
        (5, 5, 0.2),
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
        (5, 5, 0.2),
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
        .set_color(0.0, 1.0, 0.0, 1.0)
        .unwrap();
    let radius = 4f32;
    let mut i = 1;
    for edge in graph.edge_references() {
        let eps = 0.01;
        let s = graph.node_weight(edge.source()).unwrap();
        let d = graph.node_weight(edge.target()).unwrap();
        engine
            .objects
            .new_object(
                String::from("edge") + &i.to_string(),
                vec![
                    blue_engine::Vertex {
                        position: [s.x - eps, s.y - eps, s.z - eps],
                        uv: [0.0, 0.0],
                        normal: [0f32, 0f32, 0f32],
                    },
                    blue_engine::Vertex {
                        position: [s.x + eps, s.y + eps, s.z + eps],
                        uv: [0.0, 1.0],
                        normal: [0f32, 0f32, 0f32],
                    },
                    blue_engine::Vertex {
                        position: [d.x + eps, d.y + eps, d.z + eps],
                        uv: [1.0, 0.0],
                        normal: [0f32, 0f32, 0f32],
                    },
                    blue_engine::Vertex {
                        position: [d.x - eps, d.y - eps, d.z - eps],
                        uv: [1.0, 1.0],
                        normal: [0f32, 0f32, 0f32],
                    },
                ],
                vec![0, 1, 2, 0, 2, 3],
                blue_engine::ObjectSettings::default(),
                &mut engine.renderer,
            )
            .unwrap();
        engine
            .objects
            .get_mut(&(String::from("edge") + &i.to_string()))
            .unwrap()
            .set_color(0.0, 0.0, 1.0, 1.0)
            .unwrap();
        i += 1;
    }
    engine
        .update_loop(move |renderer, _, obj_storage, _, camera, _| {
            let mut neighbours = object.graph.neighbors_undirected(NodeIndex::new(
                **starting_particle.vertex_index.as_ref().unwrap() as usize,
            ));
            let chosen = neighbours.nth(0).unwrap();
            let new_vert = object.graph.node_weight(chosen).unwrap();
            starting_particle.vertex_index = Some(Box::new(chosen.index() as u32));
            starting_particle.x = new_vert.x;
            starting_particle.y = new_vert.y;
            starting_particle.z = new_vert.z;

            let rendered = obj_storage.get_mut(&starting_particle.name).unwrap();
            rendered.set_position(
                starting_particle.x,
                starting_particle.y,
                starting_particle.z,
            );
            rendered.update(renderer).unwrap();
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;

            camera
                .set_position(camx, camy, camz)
                .expect("Couldn't update the camera eye");
            // let ten_millis = time::Duration::from_millis(100);

            // thread::sleep(ten_millis);
        })
        .expect("Error during update loop");
}
