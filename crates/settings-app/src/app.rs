use crate::theme::{self, Colors};
use crate::widgets;
use iced::alignment::Vertical;
use iced::widget::{
    Column, Row, button, column, container, mouse_area, row, scrollable, space, text, text_input,
};
use iced::{Color, Element, Fill, FillPortion, Font, Task, Theme, window};
use settings_core::{Page, search};

type AppElement<'a> = Element<'a, Message>;

#[derive(Debug, Clone)]
pub enum Message {
    WindowReady(Option<window::Id>),
    Navigate(Page),
    SearchChanged(String),
    SearchSubmitted,
    ToggleTheme,
    DragWindow,
    Minimize,
    ToggleMaximize,
    Close,
    OpenNative(Page),
}

#[derive(Debug)]
pub struct App {
    page: Page,
    query: String,
    dark: bool,
    colors: Colors,
    theme: Theme,
    window_id: Option<window::Id>,
    status: String,
}

impl Default for App {
    fn default() -> Self {
        let colors = Colors::LIGHT;

        Self {
            page: Page::Home,
            query: String::new(),
            dark: false,
            colors,
            theme: theme::theme(colors),
            window_id: None,
            status: String::new(),
        }
    }
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), window::oldest().map(Message::WindowReady))
    }

    pub fn title(&self) -> String {
        settings_core::APP_NAME.to_owned()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowReady(id) => {
                self.window_id = id;
            }
            Message::Navigate(page) => {
                self.page = page;
                self.query.clear();
                self.status.clear();
            }
            Message::SearchChanged(value) => {
                self.query = value;
                self.status.clear();
            }
            Message::SearchSubmitted => {
                let results = search(&self.query);
                if results.len() == 1 {
                    self.page = results[0];
                    self.query.clear();
                }
            }
            Message::ToggleTheme => {
                self.dark = !self.dark;
                self.colors = if self.dark {
                    Colors::DARK
                } else {
                    Colors::LIGHT
                };
                self.theme = theme::theme(self.colors);
            }
            Message::DragWindow => {
                if let Some(id) = self.window_id {
                    return window::drag(id);
                }
            }
            Message::Minimize => {
                if let Some(id) = self.window_id {
                    return window::minimize(id, true);
                }
            }
            Message::ToggleMaximize => {
                if let Some(id) = self.window_id {
                    return window::toggle_maximize(id);
                }
            }
            Message::Close => {
                if let Some(id) = self.window_id {
                    return window::close(id);
                }
            }
            Message::OpenNative(page) => {
                let target = page.deep_link().to_owned();
                self.status = match open::that(&target) {
                    Ok(()) => format!("Opened {target}"),
                    Err(error) => format!("Could not open {target}: {error}"),
                };
            }
        }

        Task::none()
    }

    pub fn view(&self) -> AppElement<'_> {
        let content = self.content();
        let top_bar = self.top_bar();
        let sidebar = self.sidebar();

        container(
            column![top_bar, row![sidebar, content].height(Fill)]
                .width(Fill)
                .height(Fill),
        )
        .width(Fill)
        .height(Fill)
        .style(theme::surface(self.colors))
        .into()
    }

    fn top_bar(&self) -> AppElement<'_> {
        let colors = self.colors;

        let title = mouse_area(
            container(
                text(settings_core::APP_NAME)
                    .size(18)
                    .font(heading_font())
                    .color(colors.on_surface),
            )
            .width(Fill)
            .height(Fill)
            .align_y(Vertical::Center)
            .padding([0, 8]),
        )
        .on_press(Message::DragWindow);

        let search_bar = container(
            row![
                widgets::icon("search", 18.0, colors.on_surface_variant),
                text_input("Search settings", &self.query)
                    .on_input(Message::SearchChanged)
                    .on_submit(Message::SearchSubmitted)
                    .padding([6, 8])
                    .size(14)
                    .width(Fill)
                    .style(theme::search_input(colors)),
            ]
            .spacing(8)
            .align_y(Vertical::Center),
        )
        .padding([0, 12])
        .width(360)
        .height(40)
        .align_y(Vertical::Center)
        .style(theme::search_bar(colors));

        let theme_icon = if self.dark { "light_mode" } else { "dark_mode" };

        let theme_button = button(widgets::icon(theme_icon, 20.0, colors.on_surface_variant))
            .on_press(Message::ToggleTheme)
            .padding(8)
            .style(theme::icon_button(colors));

        let controls = row![
            window_button(
                "remove",
                Message::Minimize,
                colors.on_surface_variant,
                theme::icon_button(colors)
            ),
            window_button(
                "crop_square",
                Message::ToggleMaximize,
                colors.on_surface_variant,
                theme::icon_button(colors)
            ),
            window_button(
                "close",
                Message::Close,
                colors.on_surface_variant,
                theme::close_button(colors)
            ),
        ]
        .spacing(2);

        container(
            row![
                title,
                space::horizontal(),
                search_bar,
                theme_button,
                controls
            ]
            .spacing(12)
            .padding([0, 12])
            .height(64)
            .align_y(Vertical::Center),
        )
        .width(Fill)
        .style(theme::top_bar(colors))
        .into()
    }

    fn sidebar(&self) -> AppElement<'_> {
        let items: Vec<AppElement<'_>> = Page::ALL
            .iter()
            .map(|&page| {
                let selected = self.page == page;
                let label_color = if selected {
                    self.colors.on_primary_container
                } else {
                    self.colors.on_surface
                };

                button(
                    row![
                        widgets::icon(page.icon(), 20.0, label_color),
                        text(page.title()).size(14).color(label_color),
                    ]
                    .spacing(14)
                    .align_y(Vertical::Center),
                )
                .on_press(Message::Navigate(page))
                .width(Fill)
                .padding([10, 14])
                .style(theme::nav_item(self.colors, selected))
                .into()
            })
            .collect();

        container(scrollable(Column::with_children(items).spacing(4).padding(12)).height(Fill))
            .width(320)
            .height(Fill)
            .style(theme::sidebar(self.colors))
            .into()
    }

    fn content(&self) -> AppElement<'_> {
        let inner = if self.query.trim().is_empty() {
            if self.page == Page::Home {
                self.home()
            } else {
                self.page_view(self.page)
            }
        } else {
            self.search_results()
        };

        scrollable(
            container(
                container(inner)
                    .max_width(960)
                    .width(Fill)
                    .padding([32, 40]),
            )
            .width(Fill)
            .center_x(Fill),
        )
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn home(&self) -> AppElement<'_> {
        let colors = self.colors;
        let mut rows: Vec<AppElement<'_>> = Vec::new();

        for chunk in Page::ALL[1..].chunks(2) {
            let mut tiles: Vec<AppElement<'static>> =
                chunk.iter().map(|&page| quick_tile(page, colors)).collect();

            if tiles.len() == 1 {
                tiles.push(space::horizontal().into());
            }

            rows.push(Row::with_children(tiles).spacing(16).into());
        }

        column![
            text("Home")
                .size(32)
                .font(heading_font())
                .color(colors.on_surface),
            text("Manage your PC")
                .size(15)
                .color(colors.on_surface_variant),
            Column::with_children(rows).spacing(16),
        ]
        .spacing(12)
        .into()
    }

    fn page_view(&self, page: Page) -> AppElement<'_> {
        let colors = self.colors;
        let mut content = column![
            text(page.title())
                .size(32)
                .font(heading_font())
                .color(colors.on_surface),
            text(page.subtitle())
                .size(15)
                .color(colors.on_surface_variant),
            container(
                column![
                    text("This page is being rebuilt")
                        .size(18)
                        .font(medium_font())
                        .color(colors.on_surface),
                    text(
                        "The matching native page is available through the link below while this view is implemented."
                    )
                    .size(14)
                    .color(colors.on_surface_variant),
                    button("Open native Settings")
                        .on_press(Message::OpenNative(page))
                        .padding([10, 18])
                        .style(theme::primary_button(colors)),
                ]
                .spacing(12),
            )
            .padding(24)
            .style(theme::card(colors)),
        ]
        .spacing(12);

        if !self.status.is_empty() {
            content = content.push(text(&self.status).size(13).color(colors.on_surface_variant));
        }

        content.into()
    }

    fn search_results(&self) -> AppElement<'_> {
        let colors = self.colors;
        let results = search(&self.query);
        let mut children: Vec<AppElement<'_>> = vec![
            text("Search results")
                .size(28)
                .font(heading_font())
                .color(colors.on_surface)
                .into(),
            text(format!(
                "{} result(s) for \"{}\"",
                results.len(),
                self.query.trim()
            ))
            .size(14)
            .color(colors.on_surface_variant)
            .into(),
        ];

        if results.is_empty() {
            children.push(
                text("No settings match this search.")
                    .size(16)
                    .color(colors.on_surface)
                    .into(),
            );
        } else {
            children.extend(results.into_iter().map(|page| result_row(page, colors)));
        }

        Column::with_children(children).spacing(10).into()
    }
}

fn quick_tile(page: Page, colors: Colors) -> AppElement<'static> {
    button(
        container(
            column![
                row![
                    widgets::icon(page.icon(), 24.0, colors.primary),
                    text(page.title())
                        .size(16)
                        .font(medium_font())
                        .color(colors.on_surface),
                ]
                .spacing(12)
                .align_y(Vertical::Center),
                text(page.subtitle())
                    .size(13)
                    .color(colors.on_surface_variant),
            ]
            .spacing(8),
        )
        .width(Fill)
        .height(120)
        .padding(16)
        .style(theme::card(colors)),
    )
    .on_press(Message::Navigate(page))
    .width(FillPortion(1))
    .padding(0)
    .style(theme::card_button(colors))
    .into()
}

fn result_row(page: Page, colors: Colors) -> AppElement<'static> {
    button(
        row![
            widgets::icon(page.icon(), 22.0, colors.primary),
            column![
                text(page.title())
                    .size(15)
                    .font(medium_font())
                    .color(colors.on_surface),
                text(page.subtitle())
                    .size(12)
                    .color(colors.on_surface_variant),
            ]
            .spacing(2),
            space::horizontal(),
            widgets::icon("chevron_right", 18.0, colors.on_surface_variant),
        ]
        .spacing(14)
        .align_y(Vertical::Center)
        .width(Fill),
    )
    .on_press(Message::Navigate(page))
    .width(Fill)
    .padding([10, 14])
    .style(theme::nav_item(colors, false))
    .into()
}

fn window_button<S>(
    name: &'static str,
    message: Message,
    color: Color,
    style: S,
) -> AppElement<'static>
where
    S: Fn(&Theme, button::Status) -> button::Style + 'static,
{
    button(widgets::icon(name, 14.0, color))
        .on_press(message)
        .padding([6, 10])
        .style(style)
        .into()
}

fn heading_font() -> Font {
    Font {
        weight: iced::font::Weight::Semibold,
        ..Font::DEFAULT
    }
}

fn medium_font() -> Font {
    Font {
        weight: iced::font::Weight::Medium,
        ..Font::DEFAULT
    }
}
