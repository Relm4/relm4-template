extern crate gtk;
extern crate glib;
extern crate gio;


mod config;
mod application;
mod window;

use application::Application;

fn main () {    
    gtk::init().expect("Unable to start GTK3");

    let app = Application::new();
    app.run();
}