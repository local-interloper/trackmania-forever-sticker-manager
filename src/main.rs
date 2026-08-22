use iced::{
    Size,
    window::{self},
};


mod app_settings;
mod constants;
mod gui;
mod utils;

fn main() {
    iced::application(gui::App::new, gui::App::update, gui::App::view)
    .title("TrackMania Forever Sticker Manager")
        .window(window::Settings {
            min_size: Some(Size {
                width: 900.0,
                height: 500.0,
            }),
            decorations: true,
            maximized: true,
            ..Default::default()
        })
        .run()
        .expect("Error");
}
