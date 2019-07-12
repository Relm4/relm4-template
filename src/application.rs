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
        let app = gtk::Application::new(config::APP_ID, gio::ApplicationFlags::FLAGS_NONE).unwrap();
        let window = Window::new();

        glib::set_application_name(&format!("GtkRustTemplate{}", config::NAME_SUFFIX));
        glib::set_prgname(Some("rust-gtk-template"));

        let builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/shortcuts.ui");
        let dialog: gtk::ShortcutsWindow = builder.get_object("shortcuts").unwrap();
        window.widget.set_help_overlay(Some(&dialog));

        let application = Self { app, window };

        application.setup_gactions();
        application.setup_signals();
        application.setup_css();
        application
    }

    pub fn setup_gactions(&self) {
        // Quit
        let app = self.app.clone();
        self.add_gaction("quit", move |_, _| app.quit());
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);

        // About
        let window = self.window.widget.clone();
        self.add_gaction("about", move |_, _| {
            let builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/about_dialog.ui");
            let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();
            about_dialog.set_transient_for(&window);

            about_dialog.connect_response(|dialog, _| dialog.destroy());
            about_dialog.show();
        });
        self.app.set_accels_for_action("app.about", &["<primary>comma"]);
    }

    pub fn setup_signals(&self) {
        let window = self.window.widget.clone();
        self.app.connect_activate(move |app| {
            window.set_application(app);
            app.add_window(&window);
            window.present();
        });
    }

    fn add_gaction<F>(&self, name: &str, action: F)
    where
        for<'r, 's> F: Fn(&'r gio::SimpleAction, &'s Option<glib::Variant>) + 'static,
    {
        let simple_action = gio::SimpleAction::new(name, None);
        simple_action.connect_activate(action);
        self.app.add_action(&simple_action);
    }


    pub fn setup_css(&self) {
        let p = gtk::CssProvider::new();
        gtk::CssProvider::load_from_resource(&p, "/com/bilelmoussaoui/GtkRustTemplate/style.css");
        gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &p, 500);

    }

    pub fn run(&self) {
        info!("GtkRustTemplate{} ({})", config::NAME_SUFFIX, config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);

        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
