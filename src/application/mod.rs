mod imp;

use crate::config;
use crate::window::ExampleApplicationWindow;
use gio::ApplicationFlags;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use gtk::{StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
use log::info;

glib::wrapper! {
    pub struct ExampleApplication(ObjectSubclass<imp::ExampleApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl ExampleApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(config::APP_ID)),
            ("flags", &ApplicationFlags::empty()),
        ])
        .expect("Application initialization failed...")
    }

    fn get_main_window(&self) -> ExampleApplicationWindow {
        let imp = imp::ExampleApplication::from_instance(self);
        imp.window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.get_main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/belmoussaoui/GtkRustTemplate/style.css");
        if let Some(display) = gdk::Display::default() {
            StyleContext::add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialogBuilder::new()
            .program_name("GTK Rust Template")
            .logo_icon_name(config::APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            .website("https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template/")
            .version(config::VERSION)
            .transient_for(&self.get_main_window())
            .modal(true)
            .authors(vec!["Bilal Elmoussaoui".into()])
            .artists(vec!["Bilal Elmoussaoui".into()])
            .build();

        dialog.show();
    }

    pub fn run(&self) {
        info!("GTK Rust Template ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
