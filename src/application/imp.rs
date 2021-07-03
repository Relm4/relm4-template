use glib::WeakRef;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use log::debug;
use once_cell::sync::OnceCell;

use crate::window::ExampleApplicationWindow;

#[derive(Debug, Default)]
pub struct ExampleApplication {
    pub window: OnceCell<WeakRef<ExampleApplicationWindow>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleApplication {
    const NAME: &'static str = "ExampleApplication";
    type Type = super::ExampleApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for ExampleApplication {}

impl ApplicationImpl for ExampleApplication {
    fn activate(&self, app: &Self::Type) {
        debug!("GtkApplication<ExampleApplication>::activate");

        if let Some(window) = self.window.get() {
            let window = window.upgrade().unwrap();
            window.show();
            window.present();
            return;
        }

        app.set_resource_base_path(Some("/com/belmoussaoui/GtkRustTemplate/"));
        app.setup_css();

        let window = ExampleApplicationWindow::new(app);
        self.window
            .set(window.downgrade())
            .expect("Window already set.");

        app.setup_gactions();
        app.setup_accels();

        app.get_main_window().present();
    }

    fn startup(&self, app: &Self::Type) {
        debug!("GtkApplication<ExampleApplication>::startup");
        self.parent_startup(app);
    }
}

impl GtkApplicationImpl for ExampleApplication {}
