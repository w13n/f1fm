pub mod button {
    use iced::Color;
    use iced::Theme;
    use iced::theme::palette::Pair;
    use iced::widget::button::{Status, Style};
    use iced::{Background, Shadow, Vector, border};

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
                Shadow::default()
            },
        }
    }
    pub fn primary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.primary.strong, true),
            Status::Hovered => style(palette.primary.base, true),
            Status::Pressed => style(palette.primary.base, false),
            Status::Disabled => style(palette.primary.weak, false),
        }
    }

    pub fn secondary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.secondary.strong, true),
            Status::Hovered => style(palette.secondary.base, true),
            Status::Pressed => style(palette.secondary.base, false),
            Status::Disabled => style(palette.secondary.weak, false),
        }
    }

    pub fn success(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        match status {
            Status::Active => style(palette.success.strong, true),
            Status::Hovered => style(palette.success.base, true),
            Status::Pressed => style(palette.success.base, false),
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
    use iced::theme::palette::Extended;
    use iced::widget::text_input::{Status, Style};
    use iced::{Background, Color, Theme, border};

    fn style(ep: &Extended, border: Color) -> Style {
        Style {
            background: Background::Color(ep.background.base.color),
            border: border::rounded(5).width(2).color(border),
            icon: Color::default(),
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

pub mod pick_list {
    use iced::overlay::menu;
    use iced::theme::palette::{Extended, Pair};
    use iced::widget::pick_list::{Status, Style};
    use iced::{Background, Theme, border};

    fn style(ep: &Extended, background: Pair) -> Style {
        Style {
            text_color: background.text,
            placeholder_color: ep.secondary.weak.color,
            handle_color: ep.background.base.color,
            background: Background::Color(background.color),
            border: border::rounded(5),
        }
    }
    pub fn default(theme: &Theme, status: Status) -> Style {
        let ep = theme.extended_palette();

        match status {
            Status::Active => style(ep, ep.secondary.base),
            Status::Hovered => style(ep, ep.secondary.strong),
            Status::Opened => style(ep, ep.secondary.base),
        }
    }

    pub fn default_menu(theme: &Theme) -> menu::Style {
        let ep = theme.extended_palette();

        menu::Style {
            background: Background::Color(ep.secondary.base.color),
            border: border::rounded(5),
            text_color: ep.secondary.base.text,
            selected_text_color: ep.secondary.strong.text,
            selected_background: Background::Color(ep.secondary.strong.color),
        }
    }
}

pub mod container {
    use iced::border::Radius;
    use iced::widget::container::Style;
    use iced::{Background, Border, Shadow, Theme, border};

    pub fn content(theme: &Theme) -> Style {
        let ep = theme.extended_palette();

        Style {
            text_color: Some(ep.secondary.weak.text),
            background: Some(Background::Color(ep.secondary.weak.color)),
            border: border::rounded(5).width(2).color(ep.secondary.strong.color),
            shadow: Shadow::default(),
        }
    }

    pub fn content_title(theme: &Theme) -> Style {
        let ep = theme.extended_palette();

        Style {
            text_color: Some(ep.secondary.strong.text),
            background: Some(Background::Color(ep.secondary.strong.color)),
            border: border::rounded(5),
            shadow: Shadow::default(),
        }
    }
}
