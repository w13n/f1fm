use iced::Element;
use time::OffsetDateTime;
use crate::fantasy_season::draft::DraftChoice;
use crate::fantasy_season::FantasySeason;
use crate::fantasy_season::score::ScoreChoice;


const GRID_SIZE_DEFAULT: u8 = 20;
const TEAM_SIZE_DEFAULT: u8 = 3;

pub(super) struct Builder {
    name: String,
    teams: Vec<TeamBuilder>,
    score_choice: ScoreChoice,
    draft_choice: DraftChoice,
    season: String,
    grid_size: String,
    team_size: u8,
    enforce_uniqueness: bool,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            name: "".to_string(),
            teams: Vec::new(),
            score_choice: ScoreChoice::default(),
            draft_choice: DraftChoice::default(),
            season: (OffsetDateTime::now_utc().year() as u16).to_string(),
            grid_size: GRID_SIZE_DEFAULT.to_string(),
            team_size: TEAM_SIZE_DEFAULT,
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
            BuilderMessage::AddTeam => {
                self.teams.push(TeamBuilder::new(self.teams.size(), self.team_size))
            }
            BuilderMessage::DeleteTeam(team) => {
                self.teams.remove(team);
                for team_id in team..self.teams.size() {
                    self.teams[team_id].decrease_id()
                }
            }
            BuilderMessage::IncreaseTeamSize => {
                self.teams.iter_mut().for_each(|team| team.increase_size());
                self.team_size += 1;
            }
            BuilderMessage::DecreaseTeamSize => {
                self.teams.iter_mut().for_each(|team| team.increase_size());
                self.team_size -= 1;
            }
            BuilderMessage::ChangeDriverNum(team, index, new_driver) => {
                if validate(&new_driver) {
                    self.teams[team].change_driver(index, new_driver)
                }
            }
            BuilderMessage::ChangeSeason(season) => {
                if season.parse::<u16>().is_ok() {
                    self.season = season;
                }
            }
            BuilderMessage::ChangeGridSize(size) => {
                if size.parse::<u8>().is_ok() {
                    self.grid_size = size;
                }
            }
            BuilderMessage::ToggleEnforceUniqueness => {
                self.enforce_uniqueness = !self.enforce_uniqueness
            }
            BuilderMessage::Create => {
                panic!("create message passed to builder");
            }
            BuilderMessage::ChangeTeamName(_, _) => {}
        }
    }

    fn view(&self) -> Element<BuilderMessage> {
        todo!()
    }

    fn create(self) -> FantasySeason {
        FantasySeason::new(
            self.name,
            self.score_choice,
            self.draft_choice,
            self.teams.iter().map(|team| team.get_name()).collect(),
            self.teams.iter().map(|team| team.parse()).collect(),
            self.season.parse::<u16>().expect("cannot call create"),
            self.grid_size.parse::<u8>().expect("cannot call create"),
            self.enforce_uniqueness
        )
    }
}

enum BuilderMessage {
    ChangeName(String),
    ScoreChoiceSelected(ScoreChoice),
    DraftChoiceSelected(DraftChoice),
    AddTeam,
    DeleteTeam(usize),
    IncreaseTeamSize,
    DecreaseTeamSize,
    ChangeDriverNum(usize, usize, String),
    ChangeTeamName(usize, String),
    ChangeSeason(String),
    ChangeGridSize(String),
    ToggleEnforceUniqueness,
    Create,
}

struct TeamBuilder {
    id: usize,
    name: String,
    numbers: Vec<String>
}

impl TeamBuilder {
    fn new(id: usize, team_size: u8) -> TeamBuilder {
        let mut vec = Vec::with_capacity(team_size as usize);
        for _ in 0..team_size {
            vec.push(String::new())
        }
        TeamBuilder{
            id,
            name: String::new(),
            numbers: vec,
        }
    }

    fn view() -> Element<'static, BuilderMessage> {
        todo!()
    }

    fn decrease_id(&mut self) {
        self.id -= 1;
    }

    fn get_name(&self) -> String {
        return self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn increase_size(&mut self) {
        self.numbers.push(String::new())
    }

    fn decrease_size(&mut self) {
        self.numbers.pop();
    }

    fn change_driver(&mut self, index: usize, driver: String) {
        self.numbers[index] = driver;
    }

    fn can_parse(&self) -> bool {
        self.numbers.iter().fold(true, |can_parse, cur_val| {
            validate(cur_val) && can_parse
        })
    }

    fn parse(&self) -> Vec<u8> {
        self.numbers.iter().map(|cur_val| cur_val.parse::<u8>()
            .expect("cannot call parse if can_parse is false")).collect()
    }
}

fn validate(driver: &str) -> bool {
    driver.parse::<u8>().is_ok_and(|val| val < 100)
}