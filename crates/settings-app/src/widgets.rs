use crate::app::Message;
use crate::theme;
use iced::widget::svg;
use iced::{Color, Element};

pub fn icon(name: &str, size: f32, color: Color) -> Element<'static, Message> {
    let bytes: &'static [u8] = match name {
        "home" => include_bytes!("../../../assets/icons/home.svg").as_slice(),
        "settings" => include_bytes!("../../../assets/icons/settings.svg").as_slice(),
        "palette" => include_bytes!("../../../assets/icons/palette.svg").as_slice(),
        "apps" => include_bytes!("../../../assets/icons/apps.svg").as_slice(),
        "person" => include_bytes!("../../../assets/icons/person.svg").as_slice(),
        "schedule" => include_bytes!("../../../assets/icons/schedule.svg").as_slice(),
        "sports_esports" => include_bytes!("../../../assets/icons/sports_esports.svg").as_slice(),
        "accessibility_new" => {
            include_bytes!("../../../assets/icons/accessibility_new.svg").as_slice()
        }
        "privacy_tip" => include_bytes!("../../../assets/icons/privacy_tip.svg").as_slice(),
        "security" => include_bytes!("../../../assets/icons/security.svg").as_slice(),
        "search" => include_bytes!("../../../assets/icons/search.svg").as_slice(),
        "remove" => include_bytes!("../../../assets/icons/remove.svg").as_slice(),
        "crop_square" => include_bytes!("../../../assets/icons/crop_square.svg").as_slice(),
        "close" => include_bytes!("../../../assets/icons/close.svg").as_slice(),
        "dark_mode" => include_bytes!("../../../assets/icons/dark_mode.svg").as_slice(),
        "light_mode" => include_bytes!("../../../assets/icons/light_mode.svg").as_slice(),
        "chevron_right" => include_bytes!("../../../assets/icons/chevron_right.svg").as_slice(),
        _ => include_bytes!("../../../assets/icons/settings.svg").as_slice(),
    };

    svg(svg::Handle::from_memory(bytes))
        .width(size)
        .height(size)
        .style(theme::icon(color))
        .into()
}
