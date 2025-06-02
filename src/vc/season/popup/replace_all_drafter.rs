use crate::fantasy_season::draft;
use crate::utils::*;
use crate::vc::PADDING;
use crate::vc::season::popup::PopupMessage;
use crate::vc::style;
use iced::{Alignment, Element, Length, widget};
use std::collections::HashMap;

pub struct ReplaceAllDrafter {
    team_lineups: HashMap<String, Vec<String>>,
    enforce_uniqueness: bool,
}

impl ReplaceAllDrafter {
    pub(super) fn new(
        team_names: Vec<String>,
        team_size: usize,
        enforce_uniqueness: bool,
    ) -> ReplaceAllDrafter {
        let mut team_lineups = HashMap::new();
        for team in team_names {
            team_lineups.insert(team, vec![String::new(); team_size]);
        }

        ReplaceAllDrafter {
            team_lineups,
            enforce_uniqueness,
        }
    }

    pub(super) fn from(
        team_lineups: HashMap<String, Vec<String>>,
        enforce_uniqueness: bool,
    ) -> ReplaceAllDrafter {
        ReplaceAllDrafter {
            team_lineups,
            enforce_uniqueness,
        }
    }
    pub(super) fn view(&self) -> Element<PopupMessage> {
        let mut team_section = Vec::new();

        let mut length = 0;

        for team_name in self.team_lineups.keys() {
            length = length.max(team_name.len());
        }

        for (team_name, lineup) in &self.team_lineups {
            let mut row = Vec::new();
            row.push(
                widget::text!("{:>length$}", team_name)
                    .align_y(Alignment::Center)
                    .height(30)
                    .into(),
            );

            for (idx, num) in lineup.iter().enumerate() {
                row.push(
                    widget::text_input(&format!("#{}", idx + 1), num)
                        .style(style::text_input::default)
                        .on_input(move |num| {
                            PopupMessage::ReplaceAll(RAMessage::ChangeDriverNumber(
                                team_name.to_string(),
                                idx,
                                num,
                            ))
                        })
                        .width(50)
                        .into(),
                );
            }

            team_section.push(widget::Row::from_vec(row).spacing(PADDING).into())
        }

        widget::column![
            widget::vertical_space(),
            widget::Column::from_vec(team_section).spacing(PADDING),
            widget::vertical_space(),
            widget::button("Save")
                .on_press_maybe(self.can_draft().then_some(PopupMessage::UpdateLineup))
                .style(style::button::primary),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }

    pub(super) fn update(&mut self, message: RAMessage) {
        match message {
            RAMessage::ChangeDriverNumber(team, idx, num) => {
                if is_valid_driver_input(&num) {
                    let _ = std::mem::replace(
                        self.team_lineups
                            .get_mut(&team)
                            .unwrap()
                            .get_mut(idx)
                            .unwrap(),
                        num,
                    );
                }
            }
        }
    }

    fn can_draft(&self) -> bool {
        if self.enforce_uniqueness && !is_unique_lineups(self.team_lineups.values().flatten()) {
            return false;
        }

        self.team_lineups
            .iter()
            .all(|(_team, lineup)| lineup.iter().all(|num| is_parsable_driver(num)))
    }

    pub fn get_drafter(self) -> draft::ReplaceAll {
        if self.can_draft() {
            draft::ReplaceAll::new(
                self.team_lineups
                    .into_iter()
                    .map(|(k, v)| (k, v.iter().map(|num| num.parse::<u8>().unwrap()).collect()))
                    .collect(),
            )
        } else {
            todo!()
        }
    }
}

#[derive(Clone, Debug)]
pub enum RAMessage {
    ChangeDriverNumber(String, usize, String),
}
