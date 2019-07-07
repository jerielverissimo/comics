use crate::app::App;

use gtk::prelude::*;

impl App {
    pub fn create_actions(&self) {
        let stack = &self.window.title_bar_stack;
        let select_headbar = &self.window.select_books_headbar;
        let view_headbar = &self.window.view_books_headbar;

        {
            let btn_select_mode = &self.window.buttons["btn_select_mode"];
            btn_select_mode.connect_clicked(clone!(stack, select_headbar => move |_| {
                stack.set_visible_child(&select_headbar);
            }));
        }

        {
            let btn_cancel_select_mode = &self.window.buttons["btn_cancel_select_mode"];
            btn_cancel_select_mode.connect_clicked(clone!(stack, view_headbar => move |_| {
                stack.set_visible_child(&view_headbar);
            }));
        }
    }
}
