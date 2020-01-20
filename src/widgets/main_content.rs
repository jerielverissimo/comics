use gtk::prelude::*;

use super::book_view::BookView;

#[derive(Clone)]
pub struct MainContent {
    stack: gtk::Stack,
    book_view: BookView,
}

impl MainContent {
    pub fn new(builder: &gtk::Builder) -> Self {
        let main_content: gtk::Stack = builder
            .get_object("main_content")
            .expect("Can't find main_content in ui file!");

        Self {
            stack: main_content,
            book_view: BookView::new(builder),
        }
    }
}
