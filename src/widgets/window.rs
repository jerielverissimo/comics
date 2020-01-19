use std::collections::HashMap;

use gtk::prelude::*;
use gtk::*;

#[derive(Clone)]
pub struct Window {
    pub main_window: gtk::ApplicationWindow,
    pub buttons: HashMap<String, gtk::Button>,
    pub title_bar_stack: gtk::Stack,
    pub select_books_headbar: gtk::HeaderBar,
    pub view_books_headbar: gtk::HeaderBar,
    pub main_content: gtk::Stack,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_string(include_str!("../../res/ui/window.ui"));

        let main_window = Self::build_main_window(&builder);
        let title_bar_stack = Self::build_title_bar_stack(&builder);
        let select_books_headbar = Self::build_select_books_headbar(&builder);
        let view_books_headbar = Self::build_view_books_headbar(&builder);
        let mut buttons = HashMap::new();
        let main_content = Self::build_main_content(&builder);

        Self::add_button(&builder, &mut buttons, "btn_select_mode");
        Self::add_button(&builder, &mut buttons, "btn_cancel_select_mode");

        Window {
            main_window,
            title_bar_stack,
            select_books_headbar,
            view_books_headbar,
            buttons,
            main_content,
        }
    }

    fn build_main_window(builder: &gtk::Builder) -> gtk::ApplicationWindow {
        let main_window: gtk::ApplicationWindow = builder
            .get_object("main_window")
            .expect("Can't find main_window in ui file!");

        main_window.connect_delete_event(|_, _| {
            main_quit();
            gtk::Inhibit(false)
        });

        main_window
    }

    fn build_title_bar_stack(builder: &gtk::Builder) -> gtk::Stack {
        let title_bar_stack: gtk::Stack = builder
            .get_object("titlebar_stack")
            .expect("Can't find titlebar_stack in ui file!");

        title_bar_stack
    }

    fn build_select_books_headbar(builder: &gtk::Builder) -> gtk::HeaderBar {
        let select_books_headbar: gtk::HeaderBar = builder
            .get_object("select_books_headbar")
            .expect("Can't find select_books_headbar in ui file!");

        select_books_headbar
    }

    fn build_view_books_headbar(builder: &gtk::Builder) -> gtk::HeaderBar {
        let view_books_headbar: gtk::HeaderBar = builder
            .get_object("view_books_headbar")
            .expect("Can't find select_books_headbar in ui file!");

        view_books_headbar
    }

    fn build_main_content(builder: &gtk::Builder) -> gtk::Stack {
        let main_content: gtk::Stack = builder
            .get_object("main_content")
            .expect("Can't find main_content in ui file!");

        main_content
    }

    fn add_button(
        builder: &gtk::Builder,
        buttons: &mut HashMap<String, gtk::Button>,
        button_name: &str,
    ) {
        let btn: gtk::Button = builder
            .get_object(button_name)
            .expect(&format!("Can't find {} in ui file!", button_name));

        buttons.insert(button_name.to_owned(), btn);
    }
}
