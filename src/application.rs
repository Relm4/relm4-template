use gio::prelude::*;
use gtk::prelude::*;
use std::env;

use crate::config::APP_ID;
use crate::window::Window;

pub struct Application {
    app: gtk::Application,
    window: Window,
}

impl Application {
    pub fn new() -> Self {
        let app = gtk::Application::new(APP_ID, gio::ApplicationFlags::FLAGS_NONE).unwrap();
        let window = Window::new();

        let application = Self { app, window };

        application.setup_signals();
        application
    }

    pub fn setup_signals(&self) {
        let window = self.window.widget.clone();
        self.app.connect_activate(move |app| {
            window.set_application(app);
            app.add_window(&window);
            window.present();
        });
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
