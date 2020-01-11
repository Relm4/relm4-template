use gio::prelude::*;
use gtk::prelude::*;
use std::env;

use crate::config;
use crate::window::Window;

pub struct Application {
    app: gtk::Application,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        let app = gtk::Application::new(Some(config::APP_ID), gio::ApplicationFlags::FLAGS_NONE).unwrap();
        let window = Window::new();

        let application = Self { app, window };

        application.setup_widgets();
        application.setup_gactions();
        application.setup_signals();
        application.setup_css();
        application
    }

    fn setup_widgets(&self) {
        let builder = gtk::Builder::new_from_resource("/com/belmoussaoui/GtkRustTemplate/shortcuts.ui");
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
                let builder = gtk::Builder::new_from_resource("/com/belmoussaoui/GtkRustTemplate/about_dialog.ui");
                get_widget!(builder, gtk::AboutDialog, about_dialog);
                about_dialog.set_transient_for(Some(&window));

                about_dialog.connect_response(|dialog, _| dialog.destroy());
                about_dialog.show();

            })
        );
        self.app.set_accels_for_action("win.show-help-overlay", &["<primary>comma"]);
    }

    fn setup_signals(&self) {
        self.app.connect_activate(clone!(@weak self.window.widget as window => move |app| {
            window.set_application(Some(app));
            app.add_window(&window);
            window.present();
        }));
    }

    fn setup_css(&self) {
        let p = gtk::CssProvider::new();
        gtk::CssProvider::load_from_resource(&p, "/com/belmoussaoui/GtkRustTemplate/style.css");
        if let Some(screen) = gdk::Screen::get_default() {
            gtk::StyleContext::add_provider_for_screen(&screen, &p, 500);
        }
    }

    pub fn run(&self) {
        info!("GtkRustTemplate{} ({})", config::NAME_SUFFIX, config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
