#[macro_use]
extern crate log;

use gettextrs::*;

mod application;
#[rustfmt::skip]
mod config;
#[rustfmt::skip]
mod static_resources;
mod window;

use application::ExampleApplication;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() {
    // Initialize logger, debug is carried out via debug!, info!, and warn!.
    pretty_env_logger::init();

    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
    textdomain(GETTEXT_PACKAGE);

    gtk::glib::set_application_name("GTK Rust Template");
    gtk::glib::set_prgname(Some("gtk-rust-template"));

    gtk::init().expect("Unable to start GTK4");

    static_resources::init().expect("Failed to initialize the resource file.");

    let app = ExampleApplication::new();
    app.run();
}
