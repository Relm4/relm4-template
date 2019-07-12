extern crate gtk;
extern crate glib;
extern crate gio;


mod config;
mod static_resources;
mod application;
mod window;

use application::Application;

fn main () {    
    gtk::init().expect("Unable to start GTK3");

    static_resources::init().expect("Failed to initialize the resource file.");


    let app = Application::new();
    app.run();
}
