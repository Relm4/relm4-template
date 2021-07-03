use gio::Settings;
use glib::{signal::Inhibit, subclass::InitializingObject};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::HeaderBar;
use gtk::{gio, glib, CompositeTemplate};
use log::warn;

use crate::config::{APP_ID, PROFILE};

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/com/belmoussaoui/GtkRustTemplate/ui/window.ui")]
pub struct ExampleApplicationWindow {
    #[template_child]
    pub headerbar: TemplateChild<HeaderBar>,
    pub settings: Settings,
}

impl Default for ExampleApplicationWindow {
    fn default() -> Self {
        Self {
            headerbar: TemplateChild::default(),
            settings: Settings::new(APP_ID),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleApplicationWindow {
    const NAME: &'static str = "ExampleApplicationWindow";
    type Type = super::ExampleApplicationWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ExampleApplicationWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let builder =
            gtk::Builder::from_resource("/com/belmoussaoui/GtkRustTemplate/ui/shortcuts.ui");
        let shortcuts = builder.object("shortcuts").unwrap();
        obj.set_help_overlay(Some(&shortcuts));

        // Devel Profile
        if PROFILE == "Devel" {
            obj.style_context().add_class("devel");
        }

        // Load latest window state
        obj.load_window_size();
    }
}

impl WidgetImpl for ExampleApplicationWindow {}
impl WindowImpl for ExampleApplicationWindow {
    // Save window state on delete event
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        if let Err(err) = window.save_window_size() {
            warn!("Failed to save window state, {}", &err);
        }

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

impl ApplicationWindowImpl for ExampleApplicationWindow {}
