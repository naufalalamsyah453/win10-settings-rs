use iced::widget::{button, container, svg, text_input};
use iced::{Border, Color, Theme};

#[derive(Debug, Clone, Copy)]
pub struct Colors {
    pub surface: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub danger: Color,
}

impl Colors {
    pub const LIGHT: Self = Self {
        surface: Color::from_rgb8(0xF8, 0xF9, 0xFF),
        surface_container: Color::from_rgb8(0xEE, 0xF1, 0xF8),
        surface_container_high: Color::from_rgb8(0xE7, 0xEB, 0xF3),
        on_surface: Color::from_rgb8(0x19, 0x1C, 0x20),
        on_surface_variant: Color::from_rgb8(0x42, 0x47, 0x4E),
        primary: Color::from_rgb8(0x00, 0x61, 0xA4),
        on_primary: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        primary_container: Color::from_rgb8(0xD1, 0xE4, 0xFF),
        on_primary_container: Color::from_rgb8(0x00, 0x1D, 0x36),
        outline: Color::from_rgb8(0x72, 0x77, 0x7F),
        outline_variant: Color::from_rgb8(0xC2, 0xC7, 0xCF),
        danger: Color::from_rgb8(0xBA, 0x1A, 0x1A),
    };

    pub const DARK: Self = Self {
        surface: Color::from_rgb8(0x10, 0x14, 0x18),
        surface_container: Color::from_rgb8(0x1D, 0x20, 0x24),
        surface_container_high: Color::from_rgb8(0x27, 0x2A, 0x2F),
        on_surface: Color::from_rgb8(0xE1, 0xE2, 0xE8),
        on_surface_variant: Color::from_rgb8(0xC2, 0xC7, 0xCF),
        primary: Color::from_rgb8(0x9E, 0xCA, 0xFF),
        on_primary: Color::from_rgb8(0x00, 0x32, 0x58),
        primary_container: Color::from_rgb8(0x00, 0x49, 0x7D),
        on_primary_container: Color::from_rgb8(0xD1, 0xE4, 0xFF),
        outline: Color::from_rgb8(0x8C, 0x91, 0x99),
        outline_variant: Color::from_rgb8(0x42, 0x47, 0x4E),
        danger: Color::from_rgb8(0xFF, 0xB4, 0xAB),
    };
}

pub fn theme(colors: Colors) -> Theme {
    Theme::custom(
        "Win10 Settings",
        iced::theme::Palette {
            background: colors.surface,
            text: colors.on_surface,
            primary: colors.primary,
            success: Color::from_rgb8(0x2E, 0x7D, 0x32),
            warning: Color::from_rgb8(0xED, 0x6C, 0x02),
            danger: colors.danger,
        },
    )
}

pub fn surface(colors: Colors) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(colors.surface.into()),
        text_color: Some(colors.on_surface),
        ..Default::default()
    }
}

pub fn top_bar(colors: Colors) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(colors.surface.into()),
        text_color: Some(colors.on_surface),
        border: Border {
            color: colors.outline_variant,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn sidebar(colors: Colors) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(colors.surface_container.into()),
        text_color: Some(colors.on_surface),
        ..Default::default()
    }
}

pub fn card(colors: Colors) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(colors.surface_container_high.into()),
        text_color: Some(colors.on_surface),
        border: Border {
            color: colors.outline_variant,
            width: 1.0,
            radius: 16.0.into(),
        },
        ..Default::default()
    }
}

pub fn search_bar(colors: Colors) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(colors.surface_container_high.into()),
        text_color: Some(colors.on_surface),
        border: Border {
            color: colors.outline_variant,
            width: 1.0,
            radius: 20.0.into(),
        },
        ..Default::default()
    }
}

pub fn nav_item(
    colors: Colors,
    selected: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(if selected {
                colors.primary_container.into()
            } else if hovered {
                colors.surface_container_high.into()
            } else {
                Color::TRANSPARENT.into()
            }),
            text_color: if selected {
                colors.on_primary_container
            } else {
                colors.on_surface
            },
            border: Border {
                radius: 24.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn primary_button(colors: Colors) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(if hovered {
                Color {
                    a: 0.88,
                    ..colors.primary
                }
                .into()
            } else {
                colors.primary.into()
            }),
            text_color: colors.on_primary,
            border: Border {
                radius: 20.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn card_button(colors: Colors) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, _| button::Style {
        background: Some(Color::TRANSPARENT.into()),
        text_color: colors.on_surface,
        border: Border {
            radius: 16.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn icon_button(colors: Colors) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(if hovered {
                colors.surface_container_high.into()
            } else {
                Color::TRANSPARENT.into()
            }),
            text_color: colors.on_surface_variant,
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn close_button(colors: Colors) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);

        button::Style {
            background: Some(if hovered {
                colors.danger.into()
            } else {
                Color::TRANSPARENT.into()
            }),
            text_color: if hovered {
                Color::WHITE
            } else {
                colors.on_surface_variant
            },
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn search_input(colors: Colors) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_, status| {
        let focused = matches!(status, text_input::Status::Focused { .. });

        text_input::Style {
            background: Color::TRANSPARENT.into(),
            border: Border {
                color: if focused {
                    colors.primary
                } else {
                    Color::TRANSPARENT
                },
                width: 0.0,
                radius: 0.0.into(),
            },
            icon: colors.on_surface_variant,
            placeholder: colors.on_surface_variant,
            value: colors.on_surface,
            selection: Color {
                a: 0.3,
                ..colors.primary
            },
        }
    }
}

pub fn icon(color: Color) -> impl Fn(&Theme, svg::Status) -> svg::Style {
    move |_, _| svg::Style { color: Some(color) }
}
