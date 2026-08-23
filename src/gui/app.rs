use std::vec;

use super::widgets;

use iced::{
    Alignment, Element, Font, Length, Task, font,
    widget::{Column, button, column, container, row, scrollable, text},
};

use crate::{
    app_settings::AppSettings,
    gui::{AppMessage::ReloadStickers, tasks},
    utils,
};

pub struct App {
    settings: AppSettings,
    selected_sticker: Option<String>,
    stickers: Vec<String>,
    search: String,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    SelectSticker(String),
    ReloadStickers,
    AddSticker,
    RemoveSticker(String),
    SetSearch(String),
    ClearSearch,
    Donate,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        (
            Self {
                settings: AppSettings::load(),
                selected_sticker: None,
                stickers: vec![],
                search: String::new(),
            },
            Task::done(ReloadStickers),
        )
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            ReloadStickers => {
                self.stickers = utils::get_sticker_names(&self.settings);

                Task::none()
            }
            AppMessage::RemoveSticker(sticker_name) => {
                if let Some(selected_sticker) = &self.selected_sticker
                    && selected_sticker == &sticker_name
                {
                    self.selected_sticker = None;
                };

                let settings = self.settings.clone();
                Task::perform(
                    async move { tasks::remove_sticker(&settings, sticker_name).await },
                    |_| AppMessage::ReloadStickers,
                )
            }
            AppMessage::AddSticker => {
                let settings = self.settings.clone();

                Task::perform(
                    async move { tasks::new_sticker(&settings).await },
                    |sticker_name| sticker_name,
                )
                .then(|sticker_name| {
                    Task::done(AppMessage::ReloadStickers).chain(match sticker_name {
                        Some(sticker_name) => Task::done(AppMessage::SelectSticker(sticker_name)),
                        None => Task::none(),
                    })
                })
            }
            AppMessage::SelectSticker(sticker_name) => {
                self.selected_sticker = Some(sticker_name);

                Task::none()
            }
            AppMessage::Donate => {
                utils::handle_with_dialog(
                    open::that("https://ko-fi.com/localinterloper"),
                    "Failed to open a browser.",
                );

                Task::none()
            }
            AppMessage::SetSearch(text) => {
                self.search = text;

                Task::none()
            }
            AppMessage::ClearSearch => {
                self.search.clear();

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let mut search =
            row![iced::widget::text_input("Search", &self.search).on_input(AppMessage::SetSearch)]
                .spacing(5);

        if !self.search.is_empty() {
            search = search.extend(vec![
                button("Clear")
                    .style(button::subtle)
                    .on_press(AppMessage::ClearSearch)
                    .into(),
            ]);
        }

        column![
            widgets::title_bar(),
            row![
                column![
                    container(
                        column![
                            search,
                            scrollable(
                                Column::with_children(
                                    self.stickers
                                        .iter()
                                        .filter(|t| t
                                            .to_lowercase()
                                            .contains(&self.search.to_lowercase()))
                                        .map(move |sticker_name| {
                                            widgets::sticker_button(
                                                &self.settings,
                                                sticker_name.clone(),
                                            )
                                        }),
                                )
                                .spacing(10)
                                .padding(20),
                            )
                            .width(Length::Fill)
                        ]
                        .spacing(5)
                        .padding(5)
                    )
                    .padding(5)
                    .height(Length::Fill)
                    .width(Length::FillPortion(3))
                    .style(container::bordered_box),
                    container(
                        button(
                            row![
                                text("+").font(Font {
                                    weight: font::Weight::Bold,
                                    ..Font::default()
                                }),
                                text("Add Sticker")
                            ]
                            .spacing(5)
                            .align_y(Alignment::Center)
                        )
                        .width(Length::Fill)
                        .on_press(AppMessage::AddSticker)
                    )
                    .width(Length::Fill)
                ]
                .spacing(10),
                widgets::sticker_preview(&self.settings, self.selected_sticker.clone()),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
