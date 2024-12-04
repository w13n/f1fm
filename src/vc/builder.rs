use iced::Element;
use time::OffsetDateTime;
use crate::fantasy_season::draft::DraftChoice;
use crate::fantasy_season::score::ScoreChoice;

pub(super) struct Builder {
    name: String,
    teams: Vec<Vec<String>>,
    score_choice: ScoreChoice,
    draft_choice: DraftChoice,
    season: u16,
    grid_size: u8,
    enforce_uniqueness: bool,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            name: "".to_string(),
            teams: Vec::new(),
            score_choice: ScoreChoice::default(),
            draft_choice: DraftChoice::default(),
            season: OffsetDateTime::now_utc().year() as u16,
            grid_size: 20,
            enforce_uniqueness: true,
        }
    }

    fn update(&mut self, message: BuilderMessage) {
        match message {
            BuilderMessage::ChangeName(name) => {
                self.name = name
            }
            BuilderMessage::ScoreChoiceSelected(choice) => {
                self.score_choice = choice
            }
            BuilderMessage::DraftChoiceSelected(choice) => {
                self.draft_choice = choice
            }
            BuilderMessage::ChangeTeamCount(dir) => {
                let grid_size = self.teams.first().unwrap_or_default().len();
                let mut vec = Vec::with_capacity(grid_size);
                for i in 0..grid_size {
                    vec.push("".to_string())
                }
                self.teams.push(Vec::)
            }
            BuilderMessage::ChangeLineupCount(dir) => {}
            BuilderMessage::ChangeDriverNum(_, _, _) => {}
            BuilderMessage::ChangeSeason(dir) => {}
            BuilderMessage::ChangeGridSize(dir) => {}
            BuilderMessage::ToggleEnforceUniqueness => {}
            BuilderMessage::Create => {}
        }
    }

    fn view(&self) -> Element<BuilderMessage> {

    }
}

enum BuilderMessage {
    ChangeName(String),
    ScoreChoiceSelected(ScoreChoice),
    DraftChoiceSelected(DraftChoice),
    ChangeTeamCount(Direction),
    ChangeLineupCount(Direction),
    ChangeDriverNum(u16, u8, u8),
    ChangeSeason(Direction),
    ChangeGridSize(Direction),
    ToggleEnforceUniqueness,
    Create,
}

enum Direction {
    Increment,
    Decrement,
    IncrementLarge,
    DecrementLarge,
}