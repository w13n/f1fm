use crate::fantasy_season::draft::Skipper;
use crate::fantasy_season::FantasySeason;
use iced::widget;
use iced::widget::{text, Column};
use iced::Element;

pub struct Season {
    season: FantasySeason,
    current_round: u8,
}

impl Season {
    pub(crate) fn new(season: FantasySeason) -> Season {
        Season {
            season,
            current_round: 1,
        }
    }
    pub(crate) fn view(&self) -> Element<SeasonMessage> {
        let top_row = widget::row![
            widget::button("-").on_press_maybe(
                (!self.current_round.eq(&1)).then_some(SeasonMessage::DecrementRound)
            ),
            widget::button("+").on_press(SeasonMessage::IncrementRound),
        ];

        let leaderboard = self.season.get_points_by(self.current_round);
        let round_points = self.season.get_points_at(self.current_round);
        let round_lineup = self.season.get_lineup_at(self.current_round);

        let mut leaderboard_vec: Vec<_> = leaderboard.into_iter().collect::<Vec<_>>();
        leaderboard_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let leadership_col: Vec<_> = leaderboard_vec
            .into_iter()
            .map(|lp| text!("{:04}: {}", lp.1, lp.0).into())
            .collect();

        let status = self.season.get_status_at(self.current_round);
        let bottom_row = widget::row![
            widget::button("draft").on_press_maybe((!status.0).then_some(SeasonMessage::Draft)),
            widget::button("download")
                .on_press_maybe((!status.1).then_some(SeasonMessage::Download)),
            widget::button("score").on_press_maybe((!status.2).then_some(SeasonMessage::Score)),
        ];

        widget::column![
            top_row,
            widget::Column::from_vec(leadership_col),
            bottom_row
        ]
        .into()
    }

    pub fn update(&mut self, message: SeasonMessage) {
        match message {
            SeasonMessage::IncrementRound => {
                self.current_round += 1;
            }
            SeasonMessage::DecrementRound => {
                self.current_round -= 1;
            }
            SeasonMessage::Draft => self
                .season
                .draft(self.current_round, Box::new(Skipper::new()))
                .unwrap(),
            SeasonMessage::Download => {
                self.season.download(self.current_round).unwrap();
            }
            SeasonMessage::Score => self.season.score(self.current_round).unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SeasonMessage {
    IncrementRound,
    DecrementRound,
    Draft,
    Download,
    Score,
}
