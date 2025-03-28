use crate::fantasy_season::FantasySeason;
use iced::{Element, Task, widget};
use std::rc::Rc;

pub struct Landing {
    seasons: Vec<FantasySeason>,
}

impl Landing {
    pub fn new(seasons: Vec<FantasySeason>) -> Landing {
        Landing { seasons }
    }

    pub fn update(&mut self, message: LandingMessage) -> Task<LandingMessage> {
        match message {
            _ => panic!("unhandled landing message"),
        }
    }
    pub fn view(&self) -> Element<LandingMessage> {
        let mut col = widget::Column::from_vec(
            self.seasons
                .iter()
                .enumerate()
                .map(|(pos, season)| {
                    widget::Button::new(widget::text!("{}", season.get_name()))
                        .on_press(LandingMessage::Pick(pos))
                        .into()
                })
                .collect(),
        );

        col = col.push(
            widget::Button::new(widget::text!("Create New Season")).on_press(LandingMessage::Build),
        );

        col.into()
    }
}

#[derive(Debug, Clone)]
pub enum LandingMessage {
    Pick(usize),
    Build,
}
