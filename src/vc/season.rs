use crate::api::Api;
use crate::error::{ApiError, DownloadError};
use crate::fantasy_season::FantasySeason;
use crate::fantasy_season::draft::{DraftChoice, Skip};
use crate::fantasy_season::race_results::RaceResults;
use crate::vc::style::container::content_title;
use crate::vc::{PADDING, VCMessage, style};
use iced::widget::text::{danger, secondary};
use iced::{Alignment, Element, Font, Length};
use iced::{Task, widget};
use popup::{Popup, PopupMessage};
use std::collections::HashMap;
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

pub mod popup;

pub(super) struct Season {
    season: FantasySeason,
    current_round: u8,
    round_names: Option<HashMap<u8, String>>,
    download_attempts: HashMap<u8, String>,
    popups: Vec<Popup>,
    warning: Option<String>,
    warning_count: usize,
}

impl Season {
    pub fn get_season(&self) -> &FantasySeason {
        &self.season
    }

    pub fn take_season(self) -> FantasySeason {
        self.season
    }
    pub fn new(season: FantasySeason) -> Season {
        Season {
            season,
            current_round: 1,
            round_names: None,
            download_attempts: HashMap::new(),
            popups: Vec::new(),
            warning: None,
            warning_count: 0,
        }
    }
    pub fn view(&self) -> Element<VCMessage> {
        if !self.popups.is_empty() {
            self.popups
                .last()
                .expect("IMPOSSIBLE: CHECKED THAT POPUPS IS NOT EMPTY")
                .view()
                .map(|x| SeasonMessage::PopupMessage(x).to())
        } else {
            let top_row = self.view_top_row();
            let secondary_row = self.view_status_text();
            let bottom_row = self.view_bottom_row();
            let content_area = self.view_content_rows();

            widget::column![
                top_row,
                secondary_row,
                widget::vertical_space(),
                content_area,
                widget::vertical_space(),
                bottom_row
            ]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
        }
    }
    fn view_top_row(&self) -> widget::Row<VCMessage> {
        let exit_button = widget::button(widget::text!["exit"].align_x(Alignment::Center))
            .on_press(VCMessage::WindowExit)
            .style(style::button::secondary)
            .width(Length::Fixed(75.));

        let round_name = if let Some(round_name) = self
            .round_names
            .as_ref()
            .and_then(|hash| hash.get(&self.current_round))
        {
            widget::text!("{}", round_name)
        } else {
            widget::text!("Round {}", self.current_round)
        }
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .size(20)
        .font(Font::with_name("Formula1"));

        widget::row![
            exit_button,
            round_name,
            widget::horizontal_space().width(Length::Fixed(75.))
        ]
    }

    fn view_status_text(&self) -> widget::Text {
        if let Some(text) = &self.warning {
            widget::text!("{}", text).style(danger)
        } else {
            widget::text!(
                "{}",
                match self.season.get_status_at(self.current_round) {
                    (_, true, _) => "round results downloaded",
                    (_, false, _) => match self.download_attempts.get(&self.current_round) {
                        Some(msg) => msg,
                        None => "round results not yet downloaded",
                    },
                }
            )
            .style(secondary)
        }
    }

    fn view_content_rows(&self) -> widget::Column<VCMessage> {
        let points_by = self.season.get_points_by(self.current_round);
        let data_points_by = points_by
            .iter()
            .map(|(_, points)| points.to_string())
            .collect();
        let teams_points_by: Vec<_> = points_by.into_iter().map(|(team, _)| team).collect();
        let points_by_table =
            Self::view_table("total points", teams_points_by.clone(), data_points_by);

        let points_at = self.season.get_points_at(self.current_round);
        let points_at_table: Element<VCMessage> = match points_at {
            None => {
                let data_col = teams_points_by.iter().map(|_| "0".to_string()).collect();
                Self::view_table("points this round", teams_points_by, data_col)
            }
            Some(vec) => {
                let data_col = vec.iter().map(|(_, points)| points.to_string()).collect();
                Self::view_table(
                    "points this round",
                    vec.into_iter().map(|(team, _)| team).collect(),
                    data_col,
                )
            }
        };

        let lineup_table = if self.season.get_status_at(self.current_round).0 {
            let mut round_lineup: Vec<_> = self
                .season
                .get_lineup_at(self.current_round)
                .into_iter()
                .collect();
            round_lineup.sort();

            let data_col = round_lineup
                .iter()
                .map(|(_, points)| {
                    points.iter().fold(String::new(), |mut x, y| {
                        x.push_str(&format!(" {y:0>2}"));
                        x
                    })
                })
                .collect();
            Self::view_table(
                "lineup this round",
                round_lineup.into_iter().map(|x| x.0).collect(),
                data_col,
            )
        } else {
            let mut teams = self.season.get_team_names();
            teams.sort();
            let size = teams.len();
            Self::view_table(
                "lineup this round",
                teams,
                vec![String::from("not yet drafted"); size],
            )
        };

        widget::column![
            widget::row![points_at_table, points_by_table].spacing(PADDING),
            widget::row![lineup_table].spacing(PADDING),
        ]
        .spacing(PADDING)
        .align_x(Alignment::Center)
    }

    fn view_bottom_row(&self) -> widget::Row<VCMessage> {
        let prev_status = if self.current_round == 1 {
            (true, true, true)
        } else {
            self.season.get_status_at(self.current_round - 1)
        };
        let status = self.season.get_status_at(self.current_round);
        let next_status = self.season.get_status_at(self.current_round + 1);

        let add_button = match (prev_status, status) {
            ((false, _, _), _) => widget::button("draft"),
            (_, (false, _, _)) => widget::button("draft").on_press(SeasonMessage::DraftStart.to()),
            (_, (true, false, _)) => widget::button("score"),
            (_, (true, true, false)) => widget::button("score").on_press(SeasonMessage::Score.to()),
            (_, (true, true, true)) => widget::button("scored"),
        }
        .style(style::button::primary);

        let edit_lineup_button = match (self.current_round, status, next_status) {
            (1, _, _) => widget::button("edit lineup"),
            (_, (true, _, false), (false, _, _)) => {
                widget::button("edit lineup").on_press(SeasonMessage::ReplaceLineup.to())
            }
            _ => widget::button("edit lineup"),
        }
        .style(style::button::secondary);

        let delete_lineup_button = match (self.current_round, status, next_status) {
            (1, _, _) => widget::button("delete lineup"),
            (_, (true, _, false), (false, _, _)) => {
                widget::button("delete lineup").on_press(SeasonMessage::DeleteLineup.to())
            }
            _ => widget::button("delete lineup"),
        }
        .style(style::button::danger);

        let delete_round_button = match status {
            (_, true, _) => {
                widget::button("delete round").on_press(SeasonMessage::DeleteRound.to())
            }
            _ => widget::button("delete round"),
        }
        .style(style::button::danger);

        let left_button = widget::button("<-")
            .on_press_maybe(
                (!self.current_round.eq(&1)).then_some(SeasonMessage::DecrementRound.to()),
            )
            .style(style::button::secondary);

        let right_button = widget::button("->")
            .on_press(SeasonMessage::IncrementRound.to())
            .style(style::button::secondary);

        widget::row![
            left_button,
            widget::horizontal_space(),
            add_button,
            edit_lineup_button,
            delete_lineup_button,
            delete_round_button,
            widget::horizontal_space(),
            right_button,
        ]
        .spacing(PADDING)
    }

    fn view_table<'a>(
        title: &str,
        teams: Vec<String>,
        data: Vec<String>,
    ) -> Element<'a, VCMessage> {
        let table_width = title.width() + 2;
        let data_width_max = data.iter().map(|x| x.width()).max().unwrap_or_default();
        let title_width = (table_width - data_width_max).max(0);

        widget::container(
            widget::column![
                widget::text!("{}", title),
                widget::container(
                    widget::row![
                        widget::Column::from_iter(
                            teams
                                .into_iter()
                                .map(|x| widget::text!("{x:title_width$}").into())
                        ),
                        widget::Column::from_iter(
                            data.into_iter().map(|x| widget::text!("{x}").into())
                        )
                        .align_x(Alignment::End),
                    ]
                    .spacing(PADDING * 2)
                )
                .padding(3)
                .style(style::container::content)
            ]
            .width(Length::Shrink)
            .align_x(Alignment::Center),
        )
        .style(content_title)
        .into()
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
                        self.season.enforces_unique(),
                    ));
                    Task::none()
                }
                DraftChoice::ReplaceAll => {
                    self.popups.push(Popup::new_replace_all(
                        self.season.get_team_names(),
                        self.season.get_lineup_size() as usize,
                        self.season.enforces_unique(),
                    ));
                    Task::none()
                }
            },
            SeasonMessage::Score => {
                if let Err(se) = self.season.score(self.current_round) {
                    self.warning = Some(se.to_string());
                    self.warning_count += 1;
                    Task::perform(
                        async { tokio::time::sleep(Duration::from_secs(5)).await },
                        |_| SeasonMessage::RemoveWarning,
                    )
                } else {
                    Task::none()
                }
            }
            SeasonMessage::ReplaceLineup => {
                let team_lineups = self
                    .season
                    .get_lineup_at(self.current_round)
                    .into_iter()
                    .map(|(team, lineup)| (team, lineup.iter().map(ToString::to_string).collect()))
                    .collect();
                self.popups.push(Popup::replace_all_from(
                    team_lineups,
                    self.season.enforces_unique(),
                ));
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
                self.season.delete_lineup(self.current_round).expect(
                    "IMPOSSIBLE: UI PREVENTS THIS FROM BEING TRIGGERED WHEN METHOD WOULD ERROR",
                );
                Task::none()
            }
            SeasonMessage::DeleteRound => {
                self.season.delete_round(self.current_round).expect(
                    "IMPOSSIBLE: UI PREVENTS THIS FROM BEING TRIGGERED WHEN METHOD WOULD ERROR",
                );
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
                        .expect("IMPOSSIBLE: PM CAN ONLY TRIGGER WHEN THERE IS A POPUP");
                    Task::none()
                }
                PopupMessage::UpdateLineup => {
                    let mut drafter = self
                        .popups
                        .pop()
                        .expect("IMPOSSIBLE: PM CAN ONLY TRIGGER WHEN THERE IS A POPUP")
                        .get_drafter();
                    self.season.delete_lineup(self.current_round).expect(
                        "IMPOSSIBLE: UI PREVENTS FROM BEING TRIGGERED WHEN METHOD WOULD ERROR",
                    );
                    self.season
                        .draft(self.current_round, &mut *drafter)
                        .expect("IMPOSSIBLE: UI CANNOT CREATE AN INVALID DRAFTER");
                    Task::none()
                }
                _ => {
                    self.popups
                        .last_mut()
                        .expect("IMPOSSIBLE: PM CAN ONLY TRIGGER WHEN THERE IS A POPUP")
                        .update(pm);
                    Task::none()
                }
            },
            SeasonMessage::RemoveWarning => {
                self.warning_count -= 1;
                if self.warning_count == 0 {
                    self.warning = None;
                }

                Task::none()
            }
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
    RemoveWarning,
}

impl SeasonMessage {
    fn to(self) -> VCMessage {
        VCMessage::Season(self)
    }
}
