use gtk::prelude::*;

use crate::config::{PROFILE, NAME_SUFFIX};

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    headerbar: gtk::HeaderBar
}

impl Window {
    pub fn new () -> Self {
        let builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/window.ui");
    
        let window_widget: gtk::ApplicationWindow = builder.get_object("window").unwrap();
        
        let headerbar: gtk::HeaderBar = builder.get_object("headerbar").unwrap();

        let window = Window {
            widget: window_widget,
            headerbar: headerbar
        };

        if PROFILE == "Devel" {
            window.widget.get_style_context().add_class("devel");
        }

        window.init();
        window
    }

    pub fn init(&self) {
        self.headerbar.set_title("Fairy Tail");
    }

}
