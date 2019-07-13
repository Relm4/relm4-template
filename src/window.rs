use gio::prelude::*;
use gtk::prelude::*;

use crate::config::{APP_ID, PROFILE};

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    appmenu_button: gtk::MenuButton,
}

impl Window {
    pub fn new() -> Self {
        let settings = gio::Settings::new(APP_ID);
        let builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/window.ui");
        let window_widget: gtk::ApplicationWindow = builder.get_object("window").unwrap();
        let appmenu_btn: gtk::MenuButton = builder.get_object("appmenu_button").unwrap();

        if PROFILE == "Devel" {
            window_widget.get_style_context().add_class("devel");
        }


        let window = Window {
            widget: window_widget,
            appmenu_button: appmenu_btn,
        };

        window.init(&settings);
        window
    }

    pub fn init(&self, settings: &gio::Settings) {
        let menu_builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/menu.ui");
        let popover_menu: gtk::PopoverMenu = menu_builder.get_object("popover_menu").unwrap();
        self.appmenu_button.set_popover(Some(&popover_menu));
        self.widget.connect_delete_event(move |window, _| {

            let position = self.widget.get_position();
            let size = self.widget.get_size();
            /*
            let window_position: glib::Variant = position.to_variant();
            let window_size: glib::Variant = size.to_variant();

            settings.set_value("window-position", window_position);
            settings.set_value("window-size", window_size);
            */
            settings.set_boolean("is-maximized", window.is_maximized());

        });

        // load default position
        if settings.get_boolean("is-maximized") {
            self.widget.maximize();
        }

    }
}
