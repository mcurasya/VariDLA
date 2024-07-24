use gio::JoinHandle;
use gl;
use gl_loader;
use glib::clone;
use glium;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::drawing_area;
use gtk::AlertDialog;
use gtk::ApplicationWindow;
use gtk::DrawingArea;
use std::fs;
use std::io;
use std::thread;

// struct MainWindow {
//     //boxes
//     gtkbox_general: gtk::Box,
//     gtkbox_control: gtk::Box,
//     gtkbox_veiwer: gtk::Box,

//     //buttons
//     choose_file_button: gtk::Button,
//     exit_button: gtk::Button,
//     start_button: gtk::Button,

//     //labels
//     nb_particles_label: gtk::Label,

//     //entries
//     particles_number: gtk::Entry,

//     //checkboxes
//     show_simulation_checkbox : gtk::CheckButton

//     //radiobuttons

// }
// impl MainWindow {

// }
pub fn on_activate(application: &gtk::Application) {
    let handle:JoinHandle<()>;
    let window = gtk::ApplicationWindow::new(application);
    let exit_button = gtk::Button::builder()
        .label("Exit App")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let gtkbox_general = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let gtkbox_control = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .build();
    // let gtkbox_veiwer = gtk::Box::builder()
    //     .orientation(gtk::Orientation::Vertical)
    //     .margin_end(20)
    //     .build();
    let choose_file_button = gtk::Button::builder()
        .label("Choose file")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let show_simulation_checkbox = gtk::CheckButton::builder()
        .label("Show random movement simulation")
        .build();
    let start_button = gtk::Button::builder()
        .label("Start simulation")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let nb_particles_label = gtk::Label::builder()
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .halign(gtk::Align::Start)
        .label("Number of particles:")
        .build();
    let particles_number = gtk::Entry::builder()
        .height_request(20)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .text("1000")
        .build();
    // widget3d.connect_realize(|area| {
    //     area.make_current();
    // });
    choose_file_button.connect_clicked(clone!(@weak window => move |_| {
            println!("wtf");
            let file_chooser = gtk::FileDialog::builder().title("Choose file").accept_label("open").build();
            file_chooser.open(Some(&window), gio::Cancellable::NONE, clone!(@weak window => move |file| {
                if let Ok(file) = file {
                    let filename = file.path().expect("Couldn't get file path");
                    let input = io::BufReader::new(fs::File::open(filename).expect("error in opening file"));
                    let model:obj::Obj = obj::load_obj(input).unwrap();
                    let handle = thread::spawn(move || {
                        let sdl = sdl2::init().unwrap();
                        let video_subsystem = sdl.video().unwrap();
                        let mut event_pump = sdl.event_pump().unwrap();
                        let sim_window = video_subsystem
                            .window("Simulator", 900, 700)
                            .opengl()
                            .resizable()
                            .build()
                            .unwrap();
                        let _gl_context = sim_window.gl_create_context().unwrap();
                        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
                        let mut i = 0;
                        model.vertices;
                        'main: loop{
                            for event in event_pump.poll_iter() {
                                match event {
                                    sdl2::event::Event::Quit {..} => break 'main,
                                    _ => {},
                                }
                            }
                            unsafe {
                                i = (i+1)%2;
                                if i==1 {
                                    gl::ClearColor(0.3, 0.3, 0.5, 1.0);
                                }
                                else {
                                    gl::ClearColor(1.0, 0.1, 0.5, 1.0);
                                }
                                gl::Clear(gl::COLOR_BUFFER_BIT);
                            }
                            sim_window.gl_swap_window();
                        }
                    });
                    println!("rerendered");
                }
            }));
    }));
    start_button.connect_clicked(
        clone!(@weak window, @weak show_simulation_checkbox, @weak particles_number => move |_| {
            let text = particles_number.text();
            let nb_particles = text.parse::<u32>();
            match nb_particles {
                Ok(nb) => println!("{:#?}", nb),
                Err(e) => {
                    AlertDialog::builder().message(e.to_string()).build().show(Some(&window));
                }
            }
            println!("{:#?}", show_simulation_checkbox.is_active());
        }),
    );
    exit_button.connect_clicked(clone!(@weak window => move |_| window.close()));

    gtkbox_control.append(&choose_file_button);
    gtkbox_control.append(&nb_particles_label);
    gtkbox_control.append(&particles_number);
    gtkbox_control.append(&show_simulation_checkbox);
    gtkbox_control.append(&start_button);
    gtkbox_control.append(&exit_button);
    gtkbox_general.append(&gtkbox_control);
    // gtkbox_general.append(&gtkbox_veiwer);
    window.set_child(Some(&gtkbox_general));
    window.present();
}
