#[rustfmt::skip]
mod config;
mod app;
mod modals;
mod setup;

use gtk::gio;
use gtk::prelude::ApplicationExt;
use once_cell::unsync::Lazy;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk, RelmApp,
};

use app::App;
use setup::setup;

use crate::config::APP_ID;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

thread_local! {
    static APP: Lazy<gtk::Application> = Lazy::new(|| { gtk::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())});
}

fn main_app() -> gtk::Application {
    APP.with(|app| (*app).clone())
}

fn main() {
    setup();

    let app = main_app();
    app.set_resource_base_path(Some("/com/belmoussaoui/GtkRustTemplate/"));

    let actions = RelmActionGroup::<AppActionGroup>::new();

    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };

    actions.add_action(quit_action);

    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    app.set_action_group(Some(&actions.into_action_group()));

    let app = RelmApp::with_app(app);

    app.run::<App>(());
}
