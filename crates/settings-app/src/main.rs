mod app;
mod theme;
mod widgets;

use iced::{Size, window};

fn main() -> iced::Result {
    iced::application(app::App::boot, app::App::update, app::App::view)
        .title(app::App::title)
        .theme(app::App::theme)
        .window(window::Settings {
            size: Size::new(1120.0, 760.0),
            min_size: Some(Size::new(760.0, 540.0)),
            decorations: false,
            ..window::Settings::default()
        })
        .antialiasing(true)
        .run()
}
