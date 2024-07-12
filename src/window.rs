use gl;
use gl_loader;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::AlertDialog;
use obj;
use std::borrow::Borrow;
use std::fs;
use std::io;

pub fn on_activate(application: &gtk::Application) {
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
    let gtkbox_veiwer = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(20)
        .margin_end(20)
        .build();
    exit_button.connect_clicked(clone!(@weak window => move |_| window.close()));
    let choose_file_button = gtk::Button::builder()
        .label("Choose file")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let widget3d = gtk::GLArea::builder()
        .margin_bottom(10)
        .margin_top(10)
        .height_request(800)
        .width_request(450)
        .can_focus(true)
        .build();
    widget3d.set_auto_render(true);
    widget3d.connect_realize(|area| {
        area.make_current();
    });
    gl_loader::init_gl();
    // Load all the OpenGL function pointer using the `gl` crate.
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
    // Unload the OpenGL library.
    gl_loader::end_gl();
    widget3d.connect_render(|glarea, glcontext| {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        println!("rendered");
        gtk::glib::Propagation::Proceed
    });
    choose_file_button.connect_clicked(clone!(@weak window, @weak widget3d => move |_| {
        let file_chooser = gtk::FileDialog::builder().title("Choose file").accept_label("open").build();
        file_chooser.open(Some(&window), gio::Cancellable::NONE, move |file| {
            if let Ok(file) = file {
                let filename = file.path().expect("Couldn't get file path");
                let input = io::BufReader::new(fs::File::open(filename).expect("error in opening file"));
                let model : obj::Obj = obj::load_obj(input).expect("error in loading model");
            }
        });
    }));
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
    gtkbox_control.append(&choose_file_button);
    gtkbox_control.append(&show_simulation_checkbox);
    gtkbox_control.append(&nb_particles_label);
    gtkbox_control.append(&particles_number);

    start_button.connect_clicked(clone!(@weak window => move |_| {
        let text = particles_number.text();
        let nb_particles = text.parse::<u32>();
        match nb_particles {
            Ok(nb) => println!("{:#?}", nb),
            Err(e) => {
                AlertDialog::builder().message(e.to_string()).build().show(Some(&window));
            }
        }

        println!("{:#?}", show_simulation_checkbox.is_active());

    }));
    gtkbox_control.append(&start_button);
    gtkbox_control.append(&exit_button);
    gtkbox_veiwer.append(&widget3d);
    gtkbox_general.append(&gtkbox_control);
    gtkbox_general.append(&gtkbox_veiwer);
    window.set_child(Some(&gtkbox_general));
    window.present();
}
