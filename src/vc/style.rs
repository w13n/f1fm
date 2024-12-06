pub mod button {
    use iced::Background::Color;
    use iced::border;
    use iced::Theme;
    use iced::theme::palette::Pair;
    use iced::widget::button::{Status, Style};

    fn style(background: Pair, border: Pair) -> Style {
        Style {
            background: Some(Color(background.color)),
            text_color: background.text,
            border: border::rounded(10).color(border.color).width(5),
            shadow: Default::default(),
        }
    }
    pub fn primary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.primary.base, palette.primary.weak),
            Status::Hovered => style(palette.primary.base, palette.primary.base),
            Status::Pressed => style(palette.primary.weak, palette.primary.weak),
            Status::Disabled => style(palette.primary.base, palette.primary.base),
        }
    }
}
