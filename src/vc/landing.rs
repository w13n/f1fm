use crate::vc::VCMessage;
use iced::{Element, widget};

pub struct Landing {
    season_names: Vec<String>,
}

impl Landing {
    pub fn new(season_names: Vec<String>) -> Landing {
        Landing { season_names }
    }

    pub fn delete(&mut self, element: usize) {
        let _ = self.season_names.remove(element);
    }
    pub fn view(&self) -> Element<VCMessage> {
        let mut col = widget::Column::from_vec(
            self.season_names
                .iter()
                .enumerate()
                .map(|(pos, name)| {
                    widget::row![
                        widget::Button::new(widget::text!("{}", name))
                            .on_press(VCMessage::OpenSeason(pos)),
                        widget::Button::new(widget::text!("delete"))
                            .on_press(VCMessage::DeleteSeason(pos))
                    ]
                    .into()
                })
                .collect(),
        );

        col = col.push(
            widget::Button::new(widget::text!("Create New Season"))
                .on_press(VCMessage::OpenBuilder),
        );

        col.into()
    }
}
