#[macro_use]
extern crate log;

extern crate gio;
extern crate glib;
extern crate gtk;

use gettextrs::*;

mod application;
mod config;
mod static_resources;
mod window;
mod window_state;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() {
    gtk::init().expect("Unable to start GTK3");
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
    textdomain(GETTEXT_PACKAGE);

    static_resources::init().expect("Failed to initialize the resource file.");

    let app = Application::new();
    app.run();
}
