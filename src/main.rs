use gtk::prelude::*;
mod window;
fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id("com.github.mcurasya.VariDla")
        .build();
    app.connect_activate(window::on_activate);
    // Run the application
    app.run();
}
