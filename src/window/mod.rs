mod imp;

use gio::{ActionGroup, ActionMap};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{ApplicationWindow, Widget, Window};

use crate::application::ExampleApplication;
use crate::config::APP_ID;

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends Widget, Window, ApplicationWindow,
        @implements ActionMap, ActionGroup;
}

impl ExampleApplicationWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        let window: Self =
            glib::Object::new(&[]).expect("Failed to create ExampleApplicationWindow");
        window.set_application(Some(app));

        // Set icons for shell
        Window::set_default_icon_name(APP_ID);

        window
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = &imp::ExampleApplicationWindow::from_instance(self).settings;

        let size = self.default_size();

        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;

        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = &imp::ExampleApplicationWindow::from_instance(self).settings;

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
