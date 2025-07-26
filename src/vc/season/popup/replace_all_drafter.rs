use crate::fantasy_season::draft;
use crate::utils::*;
use crate::vc::season::popup::PopupMessage;
use crate::vc::{CONTENT, CONTENT_INPUT_PADDED, style};
use iced::{Alignment, Element, widget};
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
        let content = self
            .team_lineups
            .iter()
            .map(|(team_name, drivers)| {
                let mut row = Vec::new();

                for (idx, num) in drivers.iter().enumerate() {
                    row.push(
                        widget::text_input(&format!("#{}", idx + 1), num)
                            .size(CONTENT)
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

                (team_name.clone(), row)
            })
            .collect();

        super::lineup_view(content, self.can_draft())
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
