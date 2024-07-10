
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::AlertDialog;

fn on_activate(application: &gtk::Application) {
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
    choose_file_button.connect_clicked(clone!(@weak window => move |_| {
        let file_chooser = gtk::FileDialog::builder().title("Choose file").accept_label("open").build();

        file_chooser.open(Some(&window), gio::Cancellable::NONE, move |file| {
            if let Ok(file) = file {
                let filename = file.path().expect("Couldn't get file path");
                let contents = std::fs::read_to_string(filename).expect("Couldn't open file");
                println!("{:#?}", contents);
            }
        });
    }));
    let show_simulation_checkbox = gtk::CheckButton::builder()
        .label("show random movement simulation")
        .build();
    let start_button = gtk::Button::builder()
        .label("start simulation")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let particles_number = gtk::TextView::builder()
        .height_request(20)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    gtkbox_control.append(&choose_file_button);
    gtkbox_control.append(&show_simulation_checkbox);
    gtkbox_control.append(&particles_number);
    start_button.connect_clicked(clone!(@weak window => move |_| {
        let buffer = particles_number.buffer();
        let (start, end) = buffer.bounds();
        let text = buffer.text(&start, &end, true);
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
    gtkbox_general.append(&gtkbox_control);
    gtkbox_general.append(&gtkbox_veiwer);
    window.set_child(Some(&gtkbox_general));
    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
