use crate::fantasy_season::FantasySeason;
use crate::fantasy_season::draft::DraftChoice;
use crate::fantasy_season::score::ScoreChoice;
use crate::utils::*;
use crate::vc::VCMessage;
use iced::{Element, widget};
use time::OffsetDateTime;

const GRID_SIZE_DEFAULT: u8 = 20;
const TEAM_SIZE_DEFAULT: u8 = 2;

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
    pub fn new() -> Builder {
        Builder {
            name: String::new(),
            teams: vec![
                TeamBuilder::new(0, TEAM_SIZE_DEFAULT),
                TeamBuilder::new(1, TEAM_SIZE_DEFAULT),
                TeamBuilder::new(2, TEAM_SIZE_DEFAULT),
            ],
            score_choice: ScoreChoice::default(),
            draft_choice: DraftChoice::default(),
            season: (OffsetDateTime::now_utc().year() as u16).to_string(),
            grid_size: GRID_SIZE_DEFAULT.to_string(),
            team_size: TEAM_SIZE_DEFAULT,
            enforce_uniqueness: true,
        }
    }

    pub fn update(&mut self, message: BuilderMessage) {
        match message {
            BuilderMessage::ChangeName(name) => self.name = name,
            BuilderMessage::ScoreChoiceSelected(choice) => self.score_choice = choice,
            BuilderMessage::DraftChoiceSelected(choice) => self.draft_choice = choice,
            BuilderMessage::AddTeam => self
                .teams
                .push(TeamBuilder::new(self.teams.len(), self.team_size)),
            BuilderMessage::DeleteTeam(team) => {
                self.teams.remove(team);
                for team_id in team..self.teams.len() {
                    self.teams[team_id].decrease_id();
                }
            }
            BuilderMessage::IncreaseTeamSize => {
                self.teams.iter_mut().for_each(TeamBuilder::increase_size);
                self.team_size += 1;
            }
            BuilderMessage::DecreaseTeamSize => {
                self.teams.iter_mut().for_each(TeamBuilder::decrease_size);
                self.team_size -= 1;
            }
            BuilderMessage::ChangeDriverNum(team, index, new_driver) => {
                if is_valid_driver_input(&new_driver) {
                    self.teams[team].change_driver(index, new_driver);
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
            BuilderMessage::ToggleEnforceUniqueness(bool) => self.enforce_uniqueness = bool,
            BuilderMessage::ChangeTeamName(id, name) => self
                .teams
                .get_mut(id)
                .expect("id out of sync")
                .set_name(name),
        }
    }

    pub fn view(&self) -> Element<VCMessage> {
        let exit_button = widget::button("back").on_press(VCMessage::WindowExit);
        let name = widget::text_input("fantasy season name here", &self.name)
            .on_input(|x| BuilderMessage::ChangeName(x).to())
            .style(super::style::text_input::default);

        let team_sizes = widget::row![
            widget::button("-")
                .on_press_maybe(if self.team_size > 1 {
                    Some(BuilderMessage::DecreaseTeamSize.to())
                } else {
                    None
                })
                .style(super::style::button::secondary),
            widget::button("+")
                .on_press(BuilderMessage::IncreaseTeamSize.to())
                .style(super::style::button::secondary),
        ]
        .spacing(10);

        let add_team = widget::button("add team")
            .on_press(BuilderMessage::AddTeam.to())
            .style(super::style::button::secondary);

        let teams = widget::container(widget::scrollable(
            widget::Column::from_vec(self.teams.iter().map(|t| t.view()).collect()).spacing(10),
        ))
        .max_height(400);

        let score_coice = widget::pick_list(
            vec![
                ScoreChoice::FormulaOne,
                ScoreChoice::RacePosition,
                ScoreChoice::Improvement,
                ScoreChoice::Domination,
            ],
            Some(self.score_choice),
            |x| BuilderMessage::ScoreChoiceSelected(x).to(),
        );

        let draft_choice = widget::pick_list(
            vec![
                DraftChoice::Skip,
                DraftChoice::RollOn,
                DraftChoice::ReplaceAll,
            ],
            Some(self.draft_choice),
            |x| BuilderMessage::DraftChoiceSelected(x).to(),
        )
        .style(super::style::pick_list::default)
        .menu_style(super::style::pick_list::default_menu);

        let season = widget::text_input("season", &self.season)
            .on_input(|x| BuilderMessage::ChangeSeason(x).to())
            .style(super::style::text_input::default);

        let grid_size = widget::text_input("grid size", &self.grid_size)
            .on_input(|x| BuilderMessage::ChangeGridSize(x).to())
            .style(super::style::text_input::default);

        let uniqueness = widget::toggler(self.enforce_uniqueness)
            .on_toggle(|x| BuilderMessage::ToggleEnforceUniqueness(x).to());

        let create = widget::button("create team")
            .on_press_maybe(
                (self.teams.iter().all(TeamBuilder::can_parse)
                    && !self.teams.is_empty()
                    && (!self.enforce_uniqueness
                        || is_unique_lineups(self.teams.iter().flat_map(|x| x.iter()))))
                .then_some(VCMessage::CreateFromBuilder),
            )
            .style(super::style::button::primary);

        widget::column![
            exit_button,
            name,
            team_sizes,
            add_team,
            teams,
            score_coice,
            draft_choice,
            season,
            grid_size,
            uniqueness,
            create
        ]
        .spacing(10)
        .into()
    }

    pub fn create(&mut self) -> FantasySeason {
        FantasySeason::new(
            self.name.clone(),
            self.score_choice,
            self.draft_choice,
            self.teams
                .iter()
                .map(|team| (team.get_name(), team.parse()))
                .collect(),
            self.season.parse::<u16>().expect("cannot call create"),
            self.grid_size.parse::<u8>().expect("cannot call create"),
            self.enforce_uniqueness,
        )
    }
}

#[derive(Clone, Debug)]
pub enum BuilderMessage {
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
    ToggleEnforceUniqueness(bool),
}

impl BuilderMessage {
    fn to(self) -> VCMessage {
        VCMessage::Builder(self)
    }
}

struct TeamBuilder {
    id: usize,
    name: String,
    numbers: Vec<String>,
}

impl TeamBuilder {
    fn new(id: usize, team_size: u8) -> TeamBuilder {
        let mut vec = Vec::with_capacity(team_size as usize);
        for _ in 0..team_size {
            vec.push(String::new());
        }
        TeamBuilder {
            id,
            name: String::new(),
            numbers: vec,
        }
    }

    fn view(&self) -> Element<VCMessage> {
        let name = widget::text_input("name of team", &self.name)
            .on_input(|name| BuilderMessage::ChangeTeamName(self.id, name).to())
            .width(200)
            .style(super::style::text_input::default);

        let mut drivers = widget::Row::with_capacity(self.numbers.len());
        for idx in 0..self.numbers.len() {
            drivers = drivers
                .push(
                    widget::text_input(
                        &format!("#{}", idx + 1),
                        self.numbers.get(idx).expect("cannot happen"),
                    )
                    .style(super::style::text_input::default)
                    .on_input(move |num| BuilderMessage::ChangeDriverNum(self.id, idx, num).to())
                    .width(50),
                )
                .spacing(5);
        }

        let delete = widget::button("delete")
            .on_press(BuilderMessage::DeleteTeam(self.id).to())
            .style(super::style::button::danger);

        widget::row![name, drivers, delete].spacing(10).into()
    }

    fn decrease_id(&mut self) {
        self.id -= 1;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn increase_size(&mut self) {
        self.numbers.push(String::new());
    }

    fn decrease_size(&mut self) {
        self.numbers.pop();
    }

    fn change_driver(&mut self, index: usize, driver: String) {
        self.numbers[index] = driver;
    }

    fn can_parse(&self) -> bool {
        self.numbers.iter().all(|x| is_parsable_driver(x)) && !self.name.is_empty()
    }

    fn parse(&self) -> Vec<u8> {
        self.numbers
            .iter()
            .map(|cur_val| {
                cur_val
                    .parse::<u8>()
                    .expect("cannot call parse if can_parse is false")
            })
            .collect()
    }

    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.numbers.iter()
    }
}
