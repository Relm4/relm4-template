use crate::config;
use crate::window::Window;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk_macros::{action, get_widget};
use log::info;
use std::env;

pub struct Application {
    app: gtk::Application,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        let app =
            gtk::Application::new(Some(config::APP_ID), gio::ApplicationFlags::FLAGS_NONE).unwrap();
        let window = Window::new();

        let application = Self { app, window };

        application.setup_widgets();
        application.setup_gactions();
        application.setup_signals();
        application.setup_css();
        application
    }

    fn setup_widgets(&self) {
        let builder = gtk::Builder::from_resource("/com/belmoussaoui/GtkRustTemplate/ui/shortcuts.ui");
        get_widget!(builder, gtk::ShortcutsWindow, shortcuts);
        self.window.widget.set_help_overlay(Some(&shortcuts));
    }

    fn setup_gactions(&self) {
        // Quit
        action!(
            self.app,
            "quit",
            clone!(@strong self.app as app => move |_, _| {
                app.quit();
            })
        );
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);

        // About
        action!(
            self.app,
            "about",
            clone!(@weak self.window.widget as window => move |_, _| {
                let dialog = gtk::AboutDialogBuilder::new()
                    .program_name("GTK Rust Template")
                    .logo_icon_name(config::APP_ID)
                    // Insert your license of choice here
                    // .license_type(gtk::License::MitX11)
                    .website("https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template/")
                    .version(config::VERSION)
                    .transient_for(&window)
                    .modal(true)
                    .authors(vec!["Bilal Elmoussaoui".into()])
                    .artists(vec!["Bilal Elmoussaoui".into()])
                    .build();

                dialog.show();

            })
        );
        self.app
            .set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
    }

    fn setup_signals(&self) {
        self.app
            .connect_activate(clone!(@weak self.window.widget as window => move |app| {
                window.set_application(Some(app));
                app.add_window(&window);
                window.show_all();
            }));
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/belmoussaoui/GtkRustTemplate/style.css");
        if let Some(screen) = gdk::Screen::get_default() {
            gtk::StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    pub fn run(&self) {
        info!("GTK Rust Template ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
