mod application;
#[rustfmt::skip]
mod config;
mod window;

use application::ExampleApplication;
use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::{bindtextdomain, setlocale, textdomain, LocaleCategory};
use gio::Resource;
use gtk::{gio, glib};

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name("GTK Rust Template");
    glib::set_prgname(Some("gtk-rust-template"));

    gtk::init().expect("Unable to start GTK4");

    let res = Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = ExampleApplication::new();
    app.run();
}
