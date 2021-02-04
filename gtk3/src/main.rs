#[macro_use]
extern crate log;
#[macro_use]
extern crate glib;

use gettextrs::*;

mod application;
#[rustfmt::skip]
mod config;
mod window;
mod window_state;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};

fn main() {
    pretty_env_logger::init();
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
    textdomain(GETTEXT_PACKAGE);

    glib::set_application_name("GTK Rust Template");
    glib::set_prgname(Some("gtk-rust-template"));

    gtk::init().expect("Unable to start GTK3");

    let res = gio::Resource::load(PKGDATADIR.to_owned() + "/resources.gresource")
        .expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = Application::new();
    app.run();
}
