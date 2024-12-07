pub mod button {
    use iced::theme::palette::Pair;
    use iced::widget::button::{Status, Style};
    use iced::Color;
    use iced::Theme;
    use iced::{border, Background, Shadow, Vector};

    fn style(background: Pair, shadow: bool) -> Style {
        Style {
            background: Some(Background::Color(background.color)),
            text_color: background.text,
            border: border::rounded(5),
            shadow: if shadow {
                Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 3.0),
                    blur_radius: 1.0,
                }
            } else {
                Default::default()
            },
        }
    }
    pub fn primary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.primary.base, true),
            Status::Hovered => style(palette.primary.strong, true),
            Status::Pressed => style(palette.primary.strong, false),
            Status::Disabled => style(palette.primary.weak, false),
        }
    }

    pub fn secondary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.secondary.base, true),
            Status::Hovered => style(palette.secondary.strong, true),
            Status::Pressed => style(palette.secondary.strong, false),
            Status::Disabled => style(palette.secondary.weak, false),
        }
    }

    pub fn success(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.success.base, true),
            Status::Hovered => style(palette.success.strong, true),
            Status::Pressed => style(palette.success.strong, false),
            Status::Disabled => style(palette.success.weak, false),
        }
    }

    pub fn danger(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.danger.base, true),
            Status::Hovered => style(palette.danger.strong, true),
            Status::Pressed => style(palette.danger.strong, false),
            Status::Disabled => style(palette.danger.weak, false),
        }
    }
}

pub mod text_input {
    use iced::theme::palette::{Extended, Pair};
    use iced::theme::Palette;
    use iced::widget::text_input::{Status, Style};
    use iced::{border, Background, Color, Theme};

    fn style(ep: &Extended, border: Color) -> Style {
        Style {
            background: Background::Color(ep.background.base.color),
            border: border::rounded(5).width(2).color(border),
            icon: Default::default(),
            placeholder: ep.background.strong.color,
            value: ep.background.base.text,
            selection: ep.background.strong.color,
        }
    }
    pub fn default(theme: &Theme, status: Status) -> Style {
        let ep = theme.extended_palette();
        match status {
            Status::Active => style(ep, ep.background.strong.color),
            Status::Hovered => style(ep, ep.background.strong.color),
            Status::Focused => style(ep, ep.background.base.text),
            Status::Disabled => style(ep, ep.background.weak.color),
        }
    }
}
