use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    headerbar: gtk::HeaderBar
}

impl Window {
    pub fn new () -> Self {
        let glade_src = include_str!("main_window.ui");
        let builder = gtk::Builder::new_from_string(glade_src);
    
        let window_widget: gtk::ApplicationWindow = builder.get_object("window").unwrap();
        
        let headerbar: gtk::HeaderBar = builder.get_object("headerbar").unwrap();

        let window = Window {
            widget: window_widget,
            headerbar: headerbar
        };

        window.init();
        window
    }

    pub fn init(&self) {
        self.headerbar.set_title("Fairy Tail");
    }

}