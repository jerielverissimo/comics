use gtk::prelude::*;
use gtk::*;

pub struct Window {
    pub main_window: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_string(include_str!("../../res/ui/window.ui"));

        let main_window: gtk::ApplicationWindow = builder.get_object("main_window").expect("");

        main_window.connect_delete_event(|_, _| {
            main_quit();
            gtk::Inhibit(false)
        });

        Window { main_window }
    }
}
