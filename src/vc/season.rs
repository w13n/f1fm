use crate::api::Api;
use crate::error::{ApiError, DownloadError};
use crate::fantasy_season::FantasySeason;
use crate::fantasy_season::draft::{DraftChoice, Skip};
use crate::fantasy_season::race_results::RaceResults;
use crate::vc::style;
use iced::Element;
use iced::{Task, widget};
use popup::{Popup, PopupMessage};
use std::collections::HashMap;

pub mod popup;

pub(super) struct Season {
    season: FantasySeason,
    current_round: u8,
    round_names: Option<HashMap<u8, String>>,
    download_attempts: HashMap<u8, String>,
    popups: Vec<Popup>,
    warning: Option<String>,
}

impl Season {
    pub fn new(season: FantasySeason) -> Season {
        Season {
            season,
            current_round: 1,
            round_names: None,
            download_attempts: HashMap::new(),
            popups: Vec::new(),
            warning: None,
        }
    }
    pub fn view(&self) -> Element<SeasonMessage> {
        if !self.popups.is_empty() {
            self.popups
                .last()
                .expect("IMPOSSIBLE: CHECKED THAT POPUPS IS NOT EMPTY")
                .view()
                .map(SeasonMessage::PopupMessage)
        } else {
            let warning = widget::text!("{}", self.warning.as_ref().unwrap_or(&String::new()));
            let top = widget::text!(
                "{}",
                match self.season.get_status_at(self.current_round) {
                    (_, true, _) => "round results downloaded",
                    (_, false, _) => match self.download_attempts.get(&self.current_round) {
                        Some(msg) => msg,
                        None => "round results not yet downloaded",
                    },
                }
            );
            let round_row = widget::row![
                widget::button("-").on_press_maybe(
                    (!self.current_round.eq(&1)).then_some(SeasonMessage::DecrementRound)
                ),
                if let Some(string) = self
                    .round_names
                    .as_ref()
                    .and_then(|hash| hash.get(&self.current_round))
                {
                    widget::text!("{}", string)
                } else {
                    widget::text!("{}", self.current_round)
                },
                widget::button("+").on_press(SeasonMessage::IncrementRound),
            ];

            let leaderboard = self.season.get_points_by(self.current_round);
            let round_points = self.season.get_points_at(self.current_round);

            let leadership_col: Vec<_> = leaderboard
                .into_iter()
                .map(|tp| widget::text!("{:04}: {}", tp.1, tp.0).into())
                .collect();

            let round_col: Vec<_> = match round_points {
                None => {
                    vec![widget::text!("round not yet scored").into()]
                }
                Some(vec) => vec
                    .into_iter()
                    .map(|tp| widget::text!("{:04}: {}", tp.1, tp.0).into())
                    .collect(),
            };

            let prev_status = if self.current_round == 1 {
                (true, true, true)
            } else {
                self.season.get_status_at(self.current_round - 1)
            };
            let status = self.season.get_status_at(self.current_round);
            let next_status = self.season.get_status_at(self.current_round + 1);

            let add_button = match (prev_status, status) {
                ((false, _, _), _) => widget::button("draft"),
                ((true, _, _), (false, _, _)) => {
                    widget::button("draft").on_press(SeasonMessage::DraftStart)
                }
                ((true, _, _), (true, false, _)) => widget::button("score"),
                ((true, _, _), (true, true, false)) => {
                    widget::button("score").on_press(SeasonMessage::Score)
                }
                ((true, _, _), (true, true, true)) => widget::button("scored"),
            };

            let delete_lineup_button = match (self.current_round, status, next_status) {
                (1, _, _) => widget::button("delete lineup"),
                (_, (true, _, false), (false, _, _)) => {
                    widget::button("delete lineup").on_press(SeasonMessage::DeleteLineup)
                }
                _ => widget::button("delete lineup"),
            }
            .style(widget::button::danger);

            let edit_lineup_button = match (self.current_round, status, next_status) {
                (1, _, _) => widget::button("edit lineup"),
                (_, (true, _, false), (false, _, _)) => {
                    widget::button("edit lineup").on_press(SeasonMessage::ReplaceLineup)
                }
                _ => widget::button("edit lineup"),
            }
            .style(style::button::success);

            let delete_round_button = match status {
                (_, true, _) => widget::button("delete round").on_press(SeasonMessage::DeleteRound),
                _ => widget::button("delete round"),
            }
            .style(style::button::danger);

            let bottom_row = widget::row![
                add_button,
                delete_lineup_button,
                delete_round_button,
                edit_lineup_button
            ];

            widget::column![
                warning,
                top,
                round_row,
                widget::Column::from_vec(leadership_col),
                widget::Column::from_vec(round_col),
                bottom_row
            ]
            .into()
        }
    }

    pub fn update(&mut self, message: SeasonMessage) -> Task<SeasonMessage> {
        match message {
            SeasonMessage::IncrementRound => {
                self.current_round += 1;
                self.download_task()
            }
            SeasonMessage::DecrementRound => {
                self.current_round -= 1;
                self.download_task()
            }
            SeasonMessage::DownloadFirstRace => self.download_task(),
            SeasonMessage::DraftStart => match self.season.get_draft_choice() {
                DraftChoice::Skip => {
                    self.season
                        .draft(self.current_round, &mut Skip::new())
                        .expect("TODO");
                    Task::none()
                }
                DraftChoice::RollOn => {
                    self.popups.push(Popup::new_roll_on(
                        self.season.get_lineup_at(self.current_round - 1),
                    ));
                    Task::none()
                }
                DraftChoice::ReplaceAll => {
                    self.popups.push(Popup::new_replace_all(
                        self.season.get_team_names(),
                        self.season.get_lineup_size() as usize,
                    ));
                    Task::none()
                }
            },
            SeasonMessage::Score => {
                if let Err(se) = self.season.score(self.current_round) {
                    self.warning = Some(se.to_string())
                }
                Task::none()
            }
            SeasonMessage::ReplaceLineup => {
                let team_lineups = self
                    .season
                    .get_lineup_at(self.current_round)
                    .into_iter()
                    .map(|(team, lineup)| {
                        (team, lineup.iter().map(|num| num.to_string()).collect())
                    })
                    .collect();
                self.season
                    .delete_lineup(self.current_round)
                    .expect("IMPOSSIBLE: UI PREVENTS FROM BEING TRIGGERED WHEN METHOD WOULD ERROR");
                self.popups.push(Popup::replace_all_from(team_lineups));
                Task::none()
            }
            SeasonMessage::DownloadedResults(result) => {
                if let Ok(rr) = result.1 {
                    self.season.update_results(rr).expect("cannot happen");
                    self.download_attempts.remove(&result.0);
                } else if let Err(err) = result.1 {
                    self.download_attempts.insert(result.0, err.to_string());
                }
                Task::none()
            }
            SeasonMessage::DeleteLineup => {
                self.season
                    .delete_lineup(self.current_round)
                    .expect("IMPOSSIBLE: UI PREVENTS FROM BEING TRIGGERED WHEN METHOD WOULD ERROR");
                Task::none()
            }
            SeasonMessage::DeleteRound => {
                self.season
                    .delete_round(self.current_round)
                    .expect("IMPOSSIBLE: UI PREVENTS FROM BEING TRIGGERED WHEN METHOD WOULD ERROR");
                self.download_attempts.remove(&self.current_round);
                Task::none()
            }
            SeasonMessage::DownloadedRaceNames(results) => {
                self.round_names = results.ok();
                Task::none()
            }
            SeasonMessage::DownloadRaceNames => Task::perform(
                download_race_names(self.season.get_season()),
                SeasonMessage::DownloadedRaceNames,
            ),
            SeasonMessage::PopupMessage(pm) => match pm {
                PopupMessage::Close => {
                    self.popups
                        .pop()
                        .expect("IMPOSSIBLE: PM.C CAN ONLY TRIGGER WHEN THERE IS A POPUP");
                    Task::none()
                }
                PopupMessage::UpdateLineup => {
                    let mut drafter = self
                        .popups
                        .pop()
                        .expect("IMPOSSIBLE: PM.C CAN ONLY TRIGGER WHEN THERE IS A POPUP")
                        .get_drafter();
                    self.season
                        .draft(self.current_round, &mut *drafter)
                        .expect("IMPOSSIBLE: UI CANNOT CREATE AN INVALID DRAFTER");
                    Task::none()
                }
                _ => {
                    self.popups
                        .last_mut()
                        .expect("IMPOSSIBLE: PM.C CAN ONLY TRIGGER WHEN THERE IS A POPUP")
                        .update(pm);
                    Task::none()
                }
            },
        }
    }

    fn download_task(&mut self) -> Task<SeasonMessage> {
        if !self.season.get_status_at(self.current_round).1
            && !self.download_attempts.contains_key(&self.current_round)
        {
            self.download_attempts
                .insert(self.current_round, "round results downloading".to_string());
            Task::perform(
                build_with_round(self.current_round, self.season.get_season()),
                SeasonMessage::DownloadedResults,
            )
        } else {
            Task::none()
        }
    }
}

async fn build_with_round(round: u8, season: u16) -> (u8, Result<RaceResults, DownloadError>) {
    (round, RaceResults::build(round, season).await)
}

async fn download_race_names(season: u16) -> Result<HashMap<u8, String>, ApiError> {
    let api = Api::new();
    api.get_race_names(season).await
}

#[derive(Debug, Clone)]
pub enum SeasonMessage {
    IncrementRound,
    DecrementRound,
    DownloadFirstRace,
    DraftStart,
    Score,
    ReplaceLineup,
    DownloadedResults((u8, Result<RaceResults, DownloadError>)),
    DeleteLineup,
    DeleteRound,
    DownloadRaceNames,
    DownloadedRaceNames(Result<HashMap<u8, String>, ApiError>),
    PopupMessage(PopupMessage),
}
