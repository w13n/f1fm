use crate::fantasy_season::FantasySeason;
use crate::fantasy_season::draft::DraftChoice;
use crate::fantasy_season::score::ScoreChoice;
use crate::utils::*;
use crate::vc::{CONTENT, MONO_FONT, PADDING, VCMessage, style};
use iced::{Alignment, Element, Length, widget};
use time::OffsetDateTime;

const GRID_SIZE_DEFAULT: u8 = 20;
const TEAM_SIZE_DEFAULT: u8 = 2;

pub(super) struct Builder {
    name: String,
    teams: Vec<TeamBuilder>,
    score_choice: Option<ScoreChoice>,
    draft_choice: Option<DraftChoice>,
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
            score_choice: None,
            draft_choice: None,
            season: (OffsetDateTime::now_utc().year() as u16).to_string(),
            grid_size: GRID_SIZE_DEFAULT.to_string(),
            team_size: TEAM_SIZE_DEFAULT,
            enforce_uniqueness: true,
        }
    }

    pub fn update(&mut self, message: BuilderMessage) {
        match message {
            BuilderMessage::ChangeName(name) => self.name = name,
            BuilderMessage::ScoreChoiceSelected(choice) => self.score_choice = Some(choice),
            BuilderMessage::DraftChoiceSelected(choice) => self.draft_choice = Some(choice),
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
        let top_row = crate::vc::top_row(
            "build new season".to_string(),
            MONO_FONT,
            VCMessage::WindowExit,
        );

        let team_settings = self.view_team_settings();
        let teams = widget::container(widget::scrollable(
            widget::Column::from_vec(self.teams.iter().map(|t| t.view()).collect()).spacing(10),
        ))
        .max_height(300);
        let modes = self.view_modes();
        let season_and_grid_size = self.view_season_and_grid_size();
        let uniqueness = widget::row![
            widget::toggler(self.enforce_uniqueness)
                .label("Enforce Uniqueness")
                .on_toggle(|x| BuilderMessage::ToggleEnforceUniqueness(x).to())
                .text_size(CONTENT)
                .size(CONTENT)
        ]
        .height(Length::Shrink);
        let name = widget::text_input("fantasy season name", &self.name)
            .on_input(|x| BuilderMessage::ChangeName(x).to())
            .size(CONTENT)
            .style(style::text_input::default);
        let content = widget::column![
            team_settings,
            teams,
            widget::vertical_space().height(PADDING),
            modes,
            season_and_grid_size,
            uniqueness,
            widget::vertical_space().height(PADDING),
            name,
        ]
        .spacing(PADDING)
        .width(Length::Shrink)
        .align_x(Alignment::Center);

        let create = widget::button(widget::text!["build season"].size(CONTENT))
            .on_press_maybe(self.can_create().then_some(VCMessage::CreateFromBuilder))
            .style(style::button::primary);

        widget::column![
            top_row,
            widget::vertical_space(),
            content,
            widget::vertical_space(),
            create,
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
    fn view_team_settings(&self) -> widget::Row<VCMessage> {
        widget::row![
            widget::button(widget::text!["-"].size(CONTENT))
                .on_press_maybe(if self.team_size > 1 {
                    Some(BuilderMessage::DecreaseTeamSize.to())
                } else {
                    None
                })
                .style(style::button::secondary),
            widget::text! {"{} drivers per team", self.team_size}
                .height(Length::Fill)
                .size(CONTENT)
                .align_y(Alignment::Center),
            widget::button(widget::text!["+"].size(CONTENT))
                .on_press(BuilderMessage::IncreaseTeamSize.to())
                .style(style::button::secondary),
            widget::horizontal_space().width(PADDING),
            widget::button(widget::text!["add a team"].size(CONTENT))
                .on_press(BuilderMessage::AddTeam.to())
                .style(style::button::secondary)
        ]
        .spacing(PADDING)
        .height(Length::Shrink)
    }

    fn view_modes(&self) -> widget::Row<VCMessage> {
        let score_mode = widget::pick_list(
            vec![
                ScoreChoice::FormulaOne,
                ScoreChoice::RacePosition,
                ScoreChoice::Improvement,
                ScoreChoice::Domination,
            ],
            self.score_choice,
            |x| BuilderMessage::ScoreChoiceSelected(x).to(),
        )
        .placeholder("Score Mode")
        .style(style::pick_list::default)
        .menu_style(style::pick_list::default_menu)
        .text_size(CONTENT);

        let draft_mode = widget::pick_list(
            vec![
                DraftChoice::Skip,
                DraftChoice::RollOn,
                DraftChoice::ReplaceAll,
            ],
            self.draft_choice,
            |x| BuilderMessage::DraftChoiceSelected(x).to(),
        )
        .placeholder("Draft Mode")
        .style(style::pick_list::default)
        .menu_style(style::pick_list::default_menu)
        .text_size(CONTENT);

        widget::row![score_mode, draft_mode,]
            .spacing(PADDING)
            .height(Length::Shrink)
    }

    fn view_season_and_grid_size(&self) -> widget::Row<VCMessage> {
        widget::row![
            widget::text_input("grid size", &self.grid_size)
                .on_input(|x| BuilderMessage::ChangeGridSize(x).to())
                .align_x(Alignment::End)
                .style(style::text_input::default)
                .size(CONTENT)
                .width(35),
            widget::text!(" Drivers in ")
                .height(Length::Fill)
                .size(CONTENT)
                .align_y(Alignment::Center),
            widget::text_input("season", &self.season)
                .on_input(|x| BuilderMessage::ChangeSeason(x).to())
                .style(style::text_input::default)
                .size(CONTENT)
                .width(65),
        ]
        .height(Length::Shrink)
    }

    pub fn create(&mut self) -> FantasySeason {
        FantasySeason::new(
            self.name.clone(),
            self.score_choice.unwrap(),
            self.draft_choice.unwrap(),
            self.teams
                .iter()
                .map(|team| (team.get_name(), team.parse()))
                .collect(),
            self.season.parse::<u16>().expect("cannot call create"),
            self.grid_size.parse::<u8>().expect("cannot call create"),
            self.enforce_uniqueness,
        )
    }

    fn can_create(&self) -> bool {
        self.teams.iter().all(TeamBuilder::can_parse)
            && !self.teams.is_empty()
            && (!self.enforce_uniqueness
                || is_unique_lineups(self.teams.iter().flat_map(|x| x.iter())))
            && self.score_choice.is_some()
            && self.draft_choice.is_some()
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
            .size(CONTENT)
            .style(style::text_input::default);

        let mut drivers = widget::Row::with_capacity(self.numbers.len());
        for idx in 0..self.numbers.len() {
            drivers = drivers
                .push(
                    widget::text_input(
                        &format!("#{}", idx + 1),
                        self.numbers.get(idx).expect("cannot happen"),
                    )
                    .style(style::text_input::default)
                    .size(CONTENT)
                    .on_input(move |num| BuilderMessage::ChangeDriverNum(self.id, idx, num).to())
                    .width(50),
                )
                .spacing(5);
        }

        let delete = widget::button(widget::text!["delete"].size(CONTENT))
            .on_press(BuilderMessage::DeleteTeam(self.id).to())
            .style(style::button::danger);

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
