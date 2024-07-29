use blue_engine::{header::Engine, primitive_shapes::cube};
use classes::objtomanifold::objtomanifold;
use obj::{self};
use rfd;
mod classes;
use std::fs::File;
use std::io::BufReader;
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
    let object = objtomanifold::new(&model);
    engine
        .update_loop(move |_, _, objStorage, _, camera, _| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            objStorage.get_mut("head").unwrap();

            camera
                .set_position(camx, camy, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}
