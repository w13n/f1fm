use crate::fantasy_season::draft;
use crate::vc::season::draft_window::DWMessage;
use iced::Element;

pub(super) struct ReplaceAll {}

impl ReplaceAll {
    pub fn view(&self) -> Element<DWMessage> {
        todo!()
    }

    pub fn update(&mut self, message: RAMessage) {
        todo!()
    }

    pub fn get_drafter(self) -> draft::ReplaceAll {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub(super) enum RAMessage {}
