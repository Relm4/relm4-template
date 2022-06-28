use relm4::{
    actions::{ActionGroupName, RelmAction, RelmActionGroup},
    gtk, ComponentBuilder, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use gtk::prelude::{GtkWindowExt, SettingsExt, WidgetExt};
use gtk::{gio, glib};

use crate::config::{APP_ID, PROFILE};
use crate::modals::about::AboutDialog;

pub(super) struct App {
    about_dialog: Controller<AboutDialog>,
}

pub(super) enum AppMsg {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;
    type InitParams = ();

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About GTK Rust Template" => AboutAction,
            }
        }
    }

    view! {
        main_window = gtk::Window {
            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_end = &gtk::MenuButton {
                    set_icon_name: "open-menu-symbolic",
                    set_menu_model: Some(&primary_menu),
                }
            },

            gtk::Label {
                set_label: "Hello world!",
                add_css_class: "title-header",
            }
        }
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        _sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let about_dialog = ComponentBuilder::new()
            .launch(widgets.main_window.clone())
            .detach();

        let model = Self { about_dialog };

        let actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let main_window = widgets.main_window.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                let shortcuts = gtk::Builder::from_resource(
                    "/com/belmoussaoui/GtkRustTemplate/gtk/help-overlay.ui",
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap();
                shortcuts.set_transient_for(Some(&main_window));
                shortcuts.set_application(Some(&crate::APP.with(|app| (**app).clone())));
                shortcuts.present();
            })
        };

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(());
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);

        widgets
            .main_window
            .insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: &ComponentSender<Self>) {
        match message {
            AppMsg::Quit => todo!("Graceful shutdown"),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
