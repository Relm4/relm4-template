use gtk::prelude::GtkWindowExt;
use relm4::{component::EmptyRoot, gtk, ComponentParts, ComponentSender, SimpleComponent};

use gettextrs::gettext;

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}
pub struct AboutDialogWidgets {
    main_window: gtk::Window,
}

impl SimpleComponent for AboutDialog {
    type InitParams = gtk::Window;
    type Widgets = AboutDialogWidgets;
    type Input = ();
    type Output = ();
    type Root = EmptyRoot;

    fn init_root() -> Self::Root {
        EmptyRoot::default()
    }

    fn init(
        main_window: Self::InitParams,
        _root: &Self::Root,
        _sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = AboutDialogWidgets { main_window };

        ComponentParts { model, widgets }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: &ComponentSender<Self>) {
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template/")
            .version(VERSION)
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .transient_for(&widgets.main_window)
            .authors(vec!["Bilal Elmoussaoui".into()])
            .artists(vec!["Bilal Elmoussaoui".into()])
            .build();
        dialog.present();
    }
}
