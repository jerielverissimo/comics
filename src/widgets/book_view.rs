use gtk::prelude::*;

#[derive(Clone)]
pub struct BookView {
    gl_area: gtk::GLArea,
    nav_bar: gtk::Box,
    prev_btn: gtk::Button,
    prev_location_btn: gtk::Button,
    next_btn: gtk::Button,
    progress: gtk::Label,
    progress_bar: gtk::Scale,
}

impl BookView {
    pub fn new(builder: &gtk::Builder) -> BookView {
        let gl_area = Self::build_gl_area(&builder);
        let nav_bar = Self::build_nav_bar(&builder);
        let prev_btn = Self::build_button(&builder, "previous_btn");
        let prev_location_btn = Self::build_button(&builder, "previous_location_btn");
        let next_btn = Self::build_button(&builder, "next_btn");
        let progress = Self::build_progress(&builder);
        let progress_bar = Self::build_progress_bar(&builder);

        Self {
            gl_area,
            nav_bar,
            prev_btn,
            prev_location_btn,
            next_btn,
            progress,
            progress_bar,
        }
    }

    fn build_gl_area(builder: &gtk::Builder) -> gtk::GLArea {
        let gl_area: gtk::GLArea = builder
            .get_object("gl_area")
            .expect("Can't find gl_area in ui file!");

        gl_area
    }

    fn build_button(builder: &gtk::Builder, btn_name: &'static str) -> gtk::Button {
        let btn: gtk::Button = builder
            .get_object(btn_name)
            .expect(&format!("Can't find {} in ui file!", btn_name));

        btn
    }

    fn build_nav_bar(builder: &gtk::Builder) -> gtk::Box {
        let nav_bar: gtk::Box = builder
            .get_object("nav_bar")
            .expect("Can't find nav_bar in ui file!");

        nav_bar
    }

    fn build_progress(builder: &gtk::Builder) -> gtk::Label {
        let progress: gtk::Label = builder
            .get_object("progress")
            .expect("Can't find progress in ui file!");

        progress
    }

    fn build_progress_bar(builder: &gtk::Builder) -> gtk::Scale {
        let progress_bar: gtk::Scale = builder
            .get_object("progress_bar")
            .expect("Can't find progress_bar in ui file!");

        progress_bar
    }
}
