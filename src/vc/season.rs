use crate::api::Api;
use crate::error::DownloadError;
use crate::fantasy_season::draft::Skip;
use crate::fantasy_season::race_results::RaceResults;
use crate::fantasy_season::FantasySeason;
use crate::vc::season::SeasonMessage::{DeleteLineup, DeleteRound};
use iced::Element;
use iced::{widget, Task};
use std::collections::HashMap;

pub(super) struct Season {
    season: FantasySeason,
    current_round: u8,
    round_names: HashMap<u8, String>,
    download_attempts: HashMap<u8, bool>,
}

impl Season {
    pub fn new(season: FantasySeason) -> Season {
        let api = Api::new();
        let round_names = api.get_race_names(season.get_season()).unwrap_or_default();

        Season {
            season,
            current_round: 1,
            round_names,
            download_attempts: HashMap::new(),
        }
    }
    pub fn view(&self) -> Element<SeasonMessage> {
        let top = widget::text!(
            "{}",
            match self.season.get_status_at(self.current_round) {
                (_, true, _) => "round results downloaded",
                (_, false, _) => match self.download_attempts.get(&self.current_round) {
                    Some(bool) => match bool {
                        true => "round results downloading failed",
                        false => "round results downloading",
                    },
                    None => "round results not downloaded",
                },
            }
        );
        let round_row = widget::row![
            widget::button("-").on_press_maybe(
                (!self.current_round.eq(&1)).then_some(SeasonMessage::DecrementRound)
            ),
            if let Some(string) = self.round_names.get(&self.current_round) {
                widget::text!("{}", string)
            } else {
                widget::text!("{}", self.current_round)
            },
            widget::button("+").on_press(SeasonMessage::IncrementRound),
        ];

        let leaderboard = self.season.get_points_by(self.current_round);
        let round_points = self.season.get_points_at(self.current_round);
        let round_lineup = self.season.get_lineup_at(self.current_round);

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
            ((true, _, _), (false, _, _)) => widget::button("draft").on_press(SeasonMessage::Draft),
            ((true, _, _), (true, false, _)) => widget::button("score"),
            ((true, _, _), (true, true, false)) => {
                widget::button("score").on_press(SeasonMessage::Score)
            }
            ((true, _, _), (true, true, true)) => widget::button("scored"),
        };

        let delete_lineup_button = match (self.current_round, status, next_status) {
            (1, _, _) => widget::button("delete lineup"),
            (_, (true, _, false), (false, _, _)) => {
                widget::button("delete lineup").on_press(DeleteLineup)
            }
            _ => widget::button("delete lineup"),
        }
        .style(widget::button::danger);

        let delete_round_button = match status {
            (_, true, _) => widget::button("delete round").on_press(DeleteRound),
            _ => widget::button("delete round"),
        }
        .style(widget::button::danger);

        let bottom_row = widget::row![add_button, delete_lineup_button, delete_round_button];

        widget::column![
            top,
            round_row,
            widget::Column::from_vec(leadership_col),
            widget::Column::from_vec(round_col),
            bottom_row
        ]
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
            SeasonMessage::Draft => {
                self.season
                    .draft(self.current_round, &Skip::new())
                    .unwrap();
                Task::none()
            }
            SeasonMessage::Score => {
                self.season.score(self.current_round).unwrap();
                Task::none()
            }
            SeasonMessage::DownloadedResults(result) => {
                if let Ok(result) = result.1 {
                    self.season.update_results(result).expect("cannot happen");
                };
                self.download_attempts.insert(result.0, true);
                Task::none()
            }
            SeasonMessage::DeleteLineup => {
                self.season.delete_lineup(self.current_round).unwrap();
                Task::none()
            }
            SeasonMessage::DeleteRound => {
                self.season.delete_round(self.current_round).unwrap();
                self.download_attempts.remove(&self.current_round);
                Task::none()
            }
        }
    }

    fn download_task(&mut self) -> Task<SeasonMessage> {
        if !self.season.get_status_at(self.current_round).1
            && self.download_attempts.get(&self.current_round).is_none()
        {
            self.download_attempts.insert(self.current_round, false);
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

#[derive(Debug, Clone)]
pub enum SeasonMessage {
    IncrementRound,
    DecrementRound,
    Draft,
    Score,
    DownloadedResults((u8, Result<RaceResults, DownloadError>)),
    DeleteLineup,
    DeleteRound,
}
