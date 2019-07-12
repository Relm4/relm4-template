use gtk::prelude::*;

use crate::config::{NAME_SUFFIX, PROFILE};

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    headerbar: gtk::HeaderBar,
    appmenu_button: gtk::MenuButton,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/window.ui");

        let window_widget: gtk::ApplicationWindow = builder.get_object("window").unwrap();
        let headerbar: gtk::HeaderBar = builder.get_object("headerbar").unwrap();
        let appmenu_btn: gtk::MenuButton = builder.get_object("appmenu_button").unwrap();

        let window = Window {
            widget: window_widget,
            headerbar: headerbar,
            appmenu_button: appmenu_btn,
        };

        if PROFILE == "Devel" {
            window.widget.get_style_context().add_class("devel");
        }

        window.init();
        window
    }

    pub fn init(&self) {
        let menu_builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/menu.ui");
        let popover_menu: gtk::PopoverMenu = menu_builder.get_object("popover_menu").unwrap();
        self.appmenu_button.set_popover(Some(&popover_menu));
    }
}
