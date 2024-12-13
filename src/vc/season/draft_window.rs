use crate::fantasy_season::draft::Drafter;
use iced::application::View;
use iced::Element;
use std::collections::HashMap;

mod replace_all;
mod roll_on;

pub(super) enum DraftWindow {
    RollOn(roll_on::RollOn),
    ReplaceAll(replace_all::ReplaceAll),
}

#[derive(Clone, Debug)]
pub enum DWMessage {
    RollOn(roll_on::ROMessage),
    ReplaceAll(replace_all::RAMessage),
    Draft,
}

impl DraftWindow {
    pub fn new_roll_on(previous_lineup: HashMap<String, Vec<u8>>) -> DraftWindow {
        DraftWindow::RollOn(roll_on::RollOn::new(previous_lineup))
    }
    pub fn view(&self) -> Element<DWMessage> {
        match self {
            DraftWindow::RollOn(ro) => ro.view(),
            DraftWindow::ReplaceAll(ra) => ra.view(),
        }
    }

    pub fn update(&mut self, message: DWMessage) {
        match message {
            DWMessage::RollOn(msg) => match self {
                DraftWindow::RollOn(ro) => ro.update(msg),
                DraftWindow::ReplaceAll(_) => panic!("RollOn msg passed to ReplaceAll"),
            },
            DWMessage::ReplaceAll(msg) => match self {
                DraftWindow::RollOn(_) => panic!("ReplaceAll msg passed to ReplaceAll"),
                DraftWindow::ReplaceAll(ra) => ra.update(msg),
            },
            DWMessage::Draft => {
                panic!("draft msg passed to draft window")
            }
        }
    }

    pub fn get_drafter(self) -> Box<dyn Drafter> {
        match self {
            DraftWindow::RollOn(ro) => Box::new(ro.get_drafter()),
            DraftWindow::ReplaceAll(ra) => Box::new(ra.get_drafter()),
        }
    }
}
