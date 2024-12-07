pub mod button {
    use iced::{Background, border, Shadow, Vector};
    use iced::Color;
    use iced::Theme;
    use iced::theme::palette::Pair;
    use iced::widget::button::{Status, Style};

    fn style(background: Pair, shadow: bool) -> Style {
        Style {
            background: Some(Background::Color(background.color)),
            text_color: background.text,
            border: border::rounded(8),
            shadow: if shadow {
                Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 3.0),
                    blur_radius: 1.0,
                }
            } else {Default::default()},
        }
    }
    pub fn primary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.primary.base, true),
            Status::Hovered => style(palette.primary.strong, true),
            Status::Pressed => style(palette.primary.strong, false),
            Status::Disabled => style(palette.background.strong, false),
        }
    }
}
