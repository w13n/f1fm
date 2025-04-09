use super::PopupMessage;
use crate::fantasy_season::draft;
use crate::utils::*;
use crate::vc::style;
use iced::Element;
use iced::widget;
use std::collections::HashMap;

pub struct RollOnDrafter {
    previous_lineup: HashMap<String, Vec<u8>>,
    drivers: HashMap<String, String>,
    enforce_uniqueness: bool,
}

impl RollOnDrafter {
    pub(super) fn new(
        previous_lineup: HashMap<String, Vec<u8>>,
        enforce_uniqueness: bool,
    ) -> RollOnDrafter {
        RollOnDrafter {
            previous_lineup,
            drivers: HashMap::new(),
            enforce_uniqueness,
        }
    }
    pub(super) fn view(&self) -> Element<PopupMessage> {
        let mut draft_team = Vec::new();
        for team in self.previous_lineup.keys() {
            let mut row = Vec::new();
            row.push(widget::text!("{}", team).into());

            row.push(
                widget::text_input("#1", self.drivers.get(team).unwrap_or(&String::from("")))
                    .style(style::text_input::default)
                    .on_input(move |num| {
                        PopupMessage::RollOn(ROMessage::ChangeDriverNumber(team.to_string(), num))
                    })
                    .width(50)
                    .into(),
            );

            for driver in self.previous_lineup.get(team).unwrap().iter() {
                row.push(widget::text! {"{:02}", driver}.into())
            }
            row.pop();

            draft_team.push(widget::Row::from_vec(row).into())
        }
        draft_team.push(
            widget::button("Draft")
                .on_press_maybe(self.can_draft().then_some(PopupMessage::UpdateLineup))
                .into(),
        );

        widget::Column::from_vec(draft_team).into()
    }

    pub(super) fn update(&mut self, message: ROMessage) {
        match message {
            ROMessage::ChangeDriverNumber(team, num) => {
                if is_valid_driver_input(&num) {
                    self.drivers.insert(team, num);
                }
            }
        }
    }

    pub fn get_drafter(self) -> draft::RollOn {
        if self.can_draft() {
            draft::RollOn::new(
                self.drivers
                    .into_iter()
                    .map(|(k, v)| (k, v.parse::<u8>().unwrap()))
                    .collect(),
            )
        } else {
            todo!()
        }
    }

    fn can_draft(&self) -> bool {
        self.drivers
            .iter()
            .all(|(_team, num)| is_parsable_driver(num))
            && self.drivers.len() == self.previous_lineup.len()
            && (!self.enforce_uniqueness
                || is_unique_lineups(
                    self.previous_lineup
                        .values()
                        .flatten()
                        .copied()
                        .chain(self.drivers.values().map(|x| x.parse::<u8>().unwrap())),
                ))
    }
}

#[derive(Clone, Debug)]
pub enum ROMessage {
    ChangeDriverNumber(String, String),
}
