use iced::{Element, Length, widget};
use replace_all_drafter::ReplaceAllDrafter;
use roll_on_drafter::RollOnDrafter;
use std::collections::HashMap;
use crate::vc::style;

pub mod replace_all_drafter;
pub mod roll_on_drafter;

pub(super) enum Popup {
    RollOnDrafter(RollOnDrafter),
    ReplaceAllDrafter(ReplaceAllDrafter),
}

#[derive(Clone, Debug)]
pub enum PopupMessage {
    RollOn(roll_on_drafter::ROMessage),
    ReplaceAll(replace_all_drafter::RAMessage),
    Close,
}

impl Popup {
    pub fn new_roll_on(previous_lineup: HashMap<String, Vec<u8>>) -> Popup {
        Popup::RollOnDrafter(RollOnDrafter::new(previous_lineup))
    }

    pub fn new_replace_all(team_names: Vec<String>, team_size: usize) -> Popup {
        Popup::ReplaceAllDrafter(ReplaceAllDrafter::new(team_names, team_size))
    }

    pub fn replace_all_from(team_lineups: HashMap<String, Vec<String>>) -> Popup {
        Popup::ReplaceAllDrafter(ReplaceAllDrafter::from(team_lineups))
    }
    pub fn view(&self) -> Element<PopupMessage> {
        widget::container(
        widget::container(
            match self {
                Popup::RollOnDrafter(ro) => ro.view(),
                Popup::ReplaceAllDrafter(ra) => ra.view(),
            }
        ).style(style::container::background)).center(Length::Fill).style(style::container::overlay).into()
    }

    pub fn update(&mut self, message: PopupMessage) {
        match message {
            PopupMessage::RollOn(msg) => match self {
                Popup::RollOnDrafter(ro) => ro.update(msg),
                Popup::ReplaceAllDrafter(_) => panic!("RollOn msg passed to ReplaceAll"),
            },
            PopupMessage::ReplaceAll(msg) => match self {
                Popup::RollOnDrafter(_) => panic!("ReplaceAll msg passed to ReplaceAll"),
                Popup::ReplaceAllDrafter(ra) => ra.update(msg),
            },
            PopupMessage::Close => {
                panic!("draft msg passed to draft window")
            }
        }
    }
}
