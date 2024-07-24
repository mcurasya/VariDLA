use blue_engine::{header::Engine, primitive_shapes::cube};
use obj::{self};
use rfd;
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

    cube("Cube", &mut engine.renderer, &mut engine.objects).unwrap();
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
            model.indices,
            blue_engine::ObjectSettings {
                ..Default::default()
            },
            &mut engine.renderer,
        )
        .unwrap();
    // engine
    //     .objects
    //     .get_mut("head")
    //     .unwrap()
    //     .set_color(1f32, 1f32, 0f32, 1f32)
    //     .unwrap();
    engine
        .objects
        .get_mut("head")
        .unwrap()
        .set_position(0.0, 0.0, 0.0);
    //let texturefp = rfd::FileDialog::new()
    // .set_directory(&path)
    // .pick_file()
    // .unwrap();
    // let textur
    // let texturefp = rfd::FileDialog::new()
    // .set_directory(&path)
    // .pick_file()
    // .unwrap();
    // let texture = ImageReader::open(texturefp).unwrap().decode().unwrap();
    // engine
    //     .objects
    //     .get_mut("head")
    //     .unwrap().set_texture(texture).unwrap();
    // uv_sphere("sphere", (50, 50, 2f32), &mut engine.renderer, &mut  engine.objects).unwrap();
    // engine.objects.get_mut("sphere").unwrap().set_color(1f32, 0.0, 0.0, 1.0).unwrap();
    // engine.objects.get_mut("sphere").unwrap().set_translation(5f32, 0f32, 0f32);
    let radius = 40f32;
    let start = std::time::SystemTime::now();
    engine
        .update_loop(move |_, _, _, _, camera, _| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            camera
                .set_position(camx, camy, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}
