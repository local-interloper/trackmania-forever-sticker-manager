
use iced::{
    Alignment, Element, Length, alignment::Vertical, widget::{button, container, image, row, text},
};

use crate::{
    app_settings::AppSettings,
    gui::{AppMessage, icons},
    utils,
};

pub fn title_bar() -> Element<'static, AppMessage> {
    row![
        text("TrackMania Forever Sticker Manager").width(Length::Fill),
        row![
            button(icons::donate_icon().height(20).width(20))
                .padding(2)
                .style(button::background)
                .on_press(AppMessage::Donate)
        ]
    ]
    .align_y(Alignment::Center)
    .into()
}

pub fn sticker_button(app_settings: &AppSettings, sticker_name: String) -> Element<'_, AppMessage> {
    let image_handle: image::Handle =
        utils::get_tm_sticker_image(app_settings, &sticker_name, utils::StickerAssetType::Icon);

    let sticker_preview = image(image_handle)
        .height(32)
        .width(32)
        .content_fit(iced::ContentFit::Contain);

    let sticker_label = text(sticker_name.clone()).width(Length::Fill);

    let delete_button = button(icons::trash_icon())
        .height(32)
        .width(32)
        .style(button::subtle)
        .on_press(AppMessage::RemoveSticker(sticker_name.clone()));

    let sticker_entry = row![sticker_preview, sticker_label, delete_button]
        .width(Length::Fill)
        .spacing(5)
        .align_y(Vertical::Center)
        .padding(10);

    button(sticker_entry)
        .style(button::background)
        .on_press(AppMessage::SelectSticker(sticker_name.clone()))
        .into()
}

pub fn sticker_preview(
    app_settings: &AppSettings,
    selected_sticker: Option<String>,
) -> Element<'_, AppMessage> {
    let e: Element<'_, AppMessage> = match selected_sticker {
        Some(sticker_name) => image(utils::get_tm_sticker_image(
            app_settings,
            &sticker_name,
            utils::StickerAssetType::Sticker,
        ))
        .into(),
        None => text("Select a sticker").into(),
    };

    container(e)
        .style(container::bordered_box)
        .center(Length::FillPortion(8))
        .into()
}
