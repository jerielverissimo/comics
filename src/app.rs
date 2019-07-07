use crate::widgets::window::Window;

use gtk::prelude::*;

#[derive(Clone)]
pub(crate) struct App {
    pub window: Window,
}

impl App {
    pub fn new() -> Self {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK!");
        }

        let window = Window::new();

        let app = App { window };

        app.create_actions();

        app.window.main_window.show_all();

        gtk::main();

        app
    }
}
