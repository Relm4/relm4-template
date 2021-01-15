use gio::prelude::SettingsExt;
use gtk::prelude::GtkWindowExt;

pub fn load(window: &gtk::ApplicationWindow, settings: &gio::Settings) {
    let width = settings.get_int("window-width");
    let height = settings.get_int("window-height");

    if width > -1 && height > -1 {
        window.resize(width, height);
    }

    let is_maximized = settings.get_boolean("is-maximized");

    if is_maximized {
        window.maximize();
    }
}

pub fn save(
    window: &gtk::ApplicationWindow,
    settings: &gio::Settings,
) -> Result<(), glib::BoolError> {
    let size = window.get_size();

    settings.set_int("window-width", size.0)?;
    settings.set_int("window-height", size.1)?;

    settings.set_boolean("is-maximized", window.is_maximized())?;

    Ok(())
}
