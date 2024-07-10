use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk::glib;
use gtk::prelude::*;
use std::io;
use gio;
// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window = gtk::ApplicationWindow::new(application);
    // … with a button in it …
    let exit_button = gtk::Button::builder()
        .label("Exit App")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let gtkbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    // … which closes the window when clicked
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
    gtkbox.append(&choose_file_button);
    gtkbox.append(&exit_button);
    window.set_child(Some(&gtkbox));
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
