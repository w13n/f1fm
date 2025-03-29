use iced::{Element, Task, widget};

pub struct Landing {
    season_names: Vec<String>,
}

impl Landing {
    pub fn new(season_names: Vec<String>) -> Landing {
        Landing { season_names }
    }

    pub fn update(&mut self, message: LandingMessage) -> Task<LandingMessage> {
        panic!("unhandled landing message")
    }
    pub fn view(&self) -> Element<LandingMessage> {
        let mut col = widget::Column::from_vec(
            self.season_names
                .iter()
                .enumerate()
                .map(|(pos, name)| {
                    widget::Button::new(widget::text!("{}", name))
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
