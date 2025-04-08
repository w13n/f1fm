use iced::{Element, widget};

pub struct Landing {
    season_names: Vec<String>,
}

impl Landing {
    pub fn new(season_names: Vec<String>) -> Landing {
        Landing { season_names }
    }

    pub fn update(&mut self, message: LandingMessage) {
        match message {
            LandingMessage::Delete(usize) => {
                self.season_names.remove(usize);
            }
            _ => panic!("LM funneled upstream improperly"),
        }
    }
    pub fn view(&self) -> Element<LandingMessage> {
        let mut col = widget::Column::from_vec(
            self.season_names
                .iter()
                .enumerate()
                .map(|(pos, name)| {
                    widget::row![
                        widget::Button::new(widget::text!("{}", name))
                            .on_press(LandingMessage::Pick(pos)),
                        widget::Button::new(widget::text!("delete"))
                            .on_press(LandingMessage::Delete(pos))
                    ]
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
    Delete(usize),
    Build,
}
