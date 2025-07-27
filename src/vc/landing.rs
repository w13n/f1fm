use super::{CONTENT, SYMB_FONT, TITLE, VCAction};
use iced::{Alignment, Element, Length, widget};

use super::{PADDING, style};

pub struct Landing {
    season_names: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum LandingMessage {
    OpenBuilder,
    OpenSeason(usize),
    DeleteSeason(usize),
}

impl Landing {
    pub fn new(season_names: Vec<String>) -> Landing {
        Landing { season_names }
    }

    pub fn delete(&mut self, element: usize) {
        let _ = self.season_names.remove(element);
    }
    pub fn view(&self) -> Element<LandingMessage> {
        let content = widget::scrollable(
            widget::Column::from_vec(
                self.season_names
                    .iter()
                    .enumerate()
                    .map(|(pos, name)| {
                        widget::row![
                            widget::Button::new(widget::text!("{}", name).size(CONTENT))
                                .on_press(LandingMessage::OpenSeason(pos))
                                .style(style::button::success),
                            widget::Button::new(
                                widget::text!("\u{e872}").font(SYMB_FONT).size(CONTENT)
                            )
                            .on_press(LandingMessage::DeleteSeason(pos))
                            .style(style::button::danger)
                        ]
                        .spacing(PADDING)
                        .into()
                    })
                    .collect(),
            )
            .align_x(Alignment::End)
            .spacing(PADDING),
        );

        widget::column![
            widget::text!["welcome to F1FM: the formula one fantasy manager"]
                .size(TITLE)
                .height(TITLE + PADDING * 4)
                .align_y(Alignment::Center),
            content.height(Length::Fill),
            widget::Button::new(widget::text!("build new season").size(CONTENT))
                .on_press(LandingMessage::OpenBuilder)
                .style(style::button::primary)
        ]
        .spacing(PADDING)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }

    pub fn update(&mut self, message: LandingMessage) -> VCAction {
        match message {
            LandingMessage::OpenBuilder => VCAction::OpenBuilder,
            LandingMessage::OpenSeason(pos) => VCAction::OpenSeason(pos),
            LandingMessage::DeleteSeason(pos) => VCAction::DeleteSeason(pos),
        }
    }
}
