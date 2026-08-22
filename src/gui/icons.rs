use iced::widget::svg::Handle;
use iced::widget::{Svg, svg};
use iced::{Theme, widget};

pub fn trash_icon() -> Svg<'static> {
    fn trash_icon_style(theme: &Theme, _status: svg::Status) -> svg::Style {
        let extended_palette = theme.extended_palette();

        svg::Style {
            color: Some(extended_palette.danger.base.color),
        }
    }

    let handle = Handle::from_memory(include_bytes!("../../assets/icons/trash.svg"));

    widget::svg(handle)
        .height(32)
        .width(32)
        .opacity(0.5)
        .style(trash_icon_style)
        .content_fit(iced::ContentFit::Cover)
}

pub fn donate_icon() -> Svg<'static> {
    fn trash_icon_style(theme: &Theme, _status: svg::Status) -> svg::Style {
        let extended_palette = theme.extended_palette();

        svg::Style {
            color: Some(extended_palette.danger.base.color),
        }
    }

    let handle = Handle::from_memory(include_bytes!("../../assets/icons/donate.svg"));

    widget::svg(handle)
        .style(trash_icon_style)
        .content_fit(iced::ContentFit::Contain)
}
