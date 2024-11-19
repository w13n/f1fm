pub mod draft;
pub mod race_results;
pub mod score;
mod status;
mod team;

use crate::error::{DraftError, ResultError, ScoreError};
use crate::fantasy_season::draft::DraftChoice;
use crate::fantasy_season::score::ScoreChoice;
use draft::Drafter;
use race_results::{DriverResult, RaceResults};
use status::Status;
use std::collections::HashMap;
use team::Team;

pub struct FantasySeason {
    name: String,
    teams: Vec<Team>,
    results: Vec<RaceResults>,
    status: Status,
    score_choice: ScoreChoice,
    draft_choice: DraftChoice,
    season: u16,
    grid_size: u8,
    enforce_uniqueness: bool,
}

impl FantasySeason {
    pub fn new(
        name: String,
        score_choice: ScoreChoice,
        draft_choice: DraftChoice,
        mut team_names: Vec<String>,
        team_lineups: Vec<Vec<u8>>,
        season: u16,
        grid_size: u8,
        enforce_uniqueness: bool,
    ) -> FantasySeason {
        let team_count = team_names.len() as u16;
        let driver_count = team_lineups.first().expect("no teams exist").len() as u8;
        assert_eq!(team_names.len(), team_lineups.len());
        for lineup in &team_lineups {
            assert_eq!(lineup.len() as u8, driver_count);
        }
        if enforce_uniqueness {
            let mut already_seen =
                Vec::with_capacity((team_count * (driver_count as u16)) as usize);
            for lineup in &team_lineups {
                for driver in lineup {
                    assert!(!already_seen.contains(driver));
                    already_seen.push(*driver);
                }
            }
        }

        let mut teams = Vec::with_capacity(team_count as usize);
        for lineup in team_lineups {
            teams.push(Team::new(team_names.remove(0), lineup))
        }

        let results = Vec::new();
        let mut status = Status::new();
        status.toggle_drafted(1);

        FantasySeason {
            name,
            teams,
            results,
            status,
            score_choice,
            draft_choice,
            season,
            grid_size,
            enforce_uniqueness,
        }
    }

    pub fn download(&mut self, round: u8) -> Result<(), ResultError> {
        if self.status.has_results(round) {
            return Err(ResultError::RaceResultsAlreadyDownloaded(round));
        }

        self.results.push(RaceResults::build(round, self.season)?);
        self.status.toggle_results(round);
        Ok(())
    }

    pub fn get_score_choice(&self) -> ScoreChoice {
        self.score_choice
    }
    pub fn score(&mut self, round: u8) -> Result<(), ScoreError> {
        if !self.status.has_results(round) {
            return Err(ScoreError::RoundResultsDoNotExist(round));
        }
        if !self.status.has_drafted(round) {
            return Err(ScoreError::RoundLineupDoesNotExist(round));
        }
        if self.status.has_scored(round) {
            return Err(ScoreError::RoundResultsAlreadyExist(round));
        }

        let driver_results = &self
            .results
            .iter()
            .find(|dr| dr.round == round)
            .expect("status out of sync: DriverResult")
            .drivers;
        let mut points = Vec::with_capacity(self.teams.len());
        for team in &self.teams {
            points.push(team.calculate_score(
                round,
                self.grid_size,
                &Box::new(&self.score_choice),
                driver_results,
            )?);
        }
        for team in &mut self.teams {
            team.store_score(round, points.remove(0));
        }

        self.status.toggle_scored(round);
        Ok(())
    }

    pub fn get_draft_choice(&self) -> DraftChoice {
        self.draft_choice
    }

    pub fn draft(&mut self, round: u8, df: Box<dyn Drafter>) -> Result<(), DraftError> {
        if !self.status.has_drafted(round - 1) {
            return Err(DraftError::PreviousRoundLineupDoesNotExist(round - 1));
        }
        if self.status.has_drafted(round) {
            return Err(DraftError::RoundLineupAlreadyExists(round));
        }

        let mut lineups = Vec::with_capacity(self.teams.len());
        for team in &self.teams {
            lineups.push(team.calculate_lineup(round, &df)?);
        }

        if self.enforce_uniqueness {
            let mut already_seen =
                Vec::new();
            for lineup in &lineups {
                for driver in lineup {
                    if already_seen.contains(&driver) {
                        return Err(DraftError::RoundDraftNonUnique(round, *driver));
                    }
                    already_seen.push(driver);
                }
            }
        }

        for team in &mut self.teams {
            team.store_lineup(round, lineups.remove(0));
        }

        self.status.toggle_drafted(round);
        Ok(())
    }

    pub fn get_team_names(&self) -> Vec<String> {
        self.teams.iter().map(|t| t.name()).collect()
    }

    pub fn get_points_by(&self, round: u8) -> HashMap<String, i16> {
        let mut map = HashMap::new();
        self.teams.iter().for_each(|t| {map.insert(t.name(), t.get_points_by(round));});
        map
    }

    pub fn get_points_at(&self, round: u8) -> HashMap<String, i16> {
        let mut map = HashMap::new();
        self.teams.iter().for_each(|t| {
            if let Some(points) = t.get_points_at(round) {
                map.insert(t.name(), points);
            }
        });
        map
    }

    pub fn get_lineup_at(&self, round: u8) -> HashMap<String, Vec<u8>> {
        let mut map = HashMap::new();
        self.teams.iter().for_each(|t| {
            if let Some(points) = t.get_lineup_at(round) {
                map.insert(t.name(), points);
            }
        });
        map
    }

    pub fn get_status_at(&self, round: u8) -> (bool, bool, bool) {
        (self.status.has_drafted(round),
         self.status.has_results(round),
         self.status.has_scored(round))
    }
}
