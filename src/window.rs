use gtk::prelude::*;

use crate::config::{APP_ID, PROFILE};
use crate::window_state;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    builder: gtk::Builder,
    settings: gio::Settings,
}

impl Window {
    pub fn new() -> Self {
        let settings = gio::Settings::new(APP_ID);

        let builder = gtk::Builder::new_from_resource("/com/belmoussaoui/GtkRustTemplate/window.ui");
        get_widget!(builder, gtk::ApplicationWindow, window);

        let window_widget = Window { widget: window, builder, settings };

        window_widget.init();
        window_widget
    }

    fn init(&self) {
        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.get_style_context().add_class("devel");
        }

        // load latest window state
        window_state::load(&self.widget, &self.settings);

        // save window state on delete event
        self.widget.connect_delete_event(clone!(@strong self.settings as settings => move |window, _| {
            if let Err(err) = window_state::save(&window, &settings) {
                warn!("Failed to save window state, {}", err);
            }
            Inhibit(false)
        }));
    }
}
