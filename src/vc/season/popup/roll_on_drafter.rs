use super::PopupMessage;
use crate::fantasy_season::draft;
use crate::utils::*;
use crate::vc::season::popup::replace_all_drafter::RAMessage;
use crate::vc::style;
use iced::widget;
use iced::{Alignment, Element, Length};
use std::collections::HashMap;

pub struct RollOnDrafter {
    returning_lineup: HashMap<String, Vec<u8>>,
    drivers: HashMap<String, String>,
    enforce_uniqueness: bool,
}

impl RollOnDrafter {
    pub(super) fn new(
        mut previous_lineup: HashMap<String, Vec<u8>>,
        enforce_uniqueness: bool,
    ) -> RollOnDrafter {
        previous_lineup.values_mut().for_each(|x| {
            x.pop();
        });
        RollOnDrafter {
            returning_lineup: previous_lineup,
            drivers: HashMap::new(),
            enforce_uniqueness,
        }
    }
    pub(super) fn view(&self) -> Element<PopupMessage> {
        let content = self
            .returning_lineup
            .iter()
            .map(|(team_name, drivers)| {
                let mut row = Vec::new();

                row.push(
                    widget::text_input(
                        "#1",
                        self.drivers.get(team_name).unwrap_or(&String::from("")),
                    )
                    .style(style::text_input::default)
                    .on_input(move |num| {
                        PopupMessage::RollOn(ROMessage::ChangeDriverNumber(
                            team_name.to_string(),
                            num,
                        ))
                    })
                    .width(50)
                    .into(),
                );

                for driver in self.returning_lineup.get(team_name).unwrap() {
                    row.push(
                        widget::text! {"{:02}", driver}
                            .align_y(Alignment::Center)
                            .height(30)
                            .into(),
                    );
                }

                (team_name.clone(), row)
            })
            .collect();

        super::lineup_view(content, self.can_draft())
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
            && self.drivers.len() == self.returning_lineup.len()
            && (!self.enforce_uniqueness
                || is_unique_lineups(
                    self.returning_lineup
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
