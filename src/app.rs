use crate::widgets::window::Window;

use gtk::prelude::*;

pub(crate) struct App {
    _window: Window,
}

impl App {
    pub fn new() -> Self {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK!");
        }

        let window = Window::new();

        window.main_window.show_all();

        gtk::main();

        App { _window: window }
    }
}
