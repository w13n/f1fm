use iced::application::View;
use iced::Element;

mod replace_all;
mod roll_on;

pub(super) enum DraftWindow {
    RollOn(roll_on::RollOn),
    ReplaceAll(replace_all::ReplaceAll),
}

pub enum DWMessage {
    RollOn(roll_on::ROMessage),
    ReplaceAll(replace_all::RAMessage),
}

impl DraftWindow {
    fn view(&self) -> Element<DWMessage> {
        match self {
            DraftWindow::RollOn(ro) => ro.view().map(DWMessage::RollOn),
            DraftWindow::ReplaceAll(ra) => ra.view().map(DWMessage::ReplaceAll),
        }
    }

    fn update(&mut self, message: DWMessage) {
        match self {
            DraftWindow::RollOn(ro) => match message {
                DWMessage::RollOn(msg) => ro.update(msg),
                DWMessage::ReplaceAll(_) => {
                    panic!("ReplaceAll message passed to RollOn")
                }
            },
            DraftWindow::ReplaceAll(ra) => match message {
                DWMessage::RollOn(_) => {
                    panic!("ReplaceAll message passed to RollOn")
                }
                DWMessage::ReplaceAll(msg) => ra.update(msg),
            },
        }
    }
}
