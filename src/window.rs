use gtk::prelude::*;

use crate::config::{APP_ID, PROFILE};
use crate::window_state;

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

        window.init(settings);
        window
    }

    pub fn init(&self, settings: gio::Settings) {
        // setup app menu
        let menu_builder = gtk::Builder::new_from_resource("/com/bilelmoussaoui/GtkRustTemplate/menu.ui");
        let popover_menu: gtk::PopoverMenu = menu_builder.get_object("popover_menu").unwrap();
        self.appmenu_button.set_popover(Some(&popover_menu));
        // load latest window state
        window_state::load(&self.widget, &settings);

        // save window state on delete event
        self.widget.connect_delete_event(move |window, _| {
            window_state::save(&window, &settings);
            Inhibit(false)
        });
    }
}
