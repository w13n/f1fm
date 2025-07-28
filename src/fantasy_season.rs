pub mod draft;
pub mod error;
pub mod race_results;
pub mod score;
mod status;
mod team;

use draft::{DraftChoice, Drafter};
use error::{DeleteError, DownloadError, DraftError, ScoreError};
use race_results::{DriverResult, RaceResults};
use score::ScoreChoice;
use serde::{Deserialize, Serialize};
use status::Status;
use std::collections::{HashMap, HashSet};
use team::Team;

#[derive(Debug, Serialize, Deserialize)]
pub struct FantasySeason {
    name: String,
    teams: Vec<Team>,
    results: Vec<RaceResults>,
    status: Status,
    score_choice: ScoreChoice,
    draft_choice: DraftChoice,
    lineup_size: u8,
    season: u16,
    grid_size: u8,
    enforce_uniqueness: bool,
}

impl FantasySeason {
    pub fn new<I: IntoIterator<Item = String>>(
        name: String,
        score_choice: ScoreChoice,
        draft_choice: DraftChoice,
        starting_teams: I,
        lineup_size: u8,
        season: u16,
        grid_size: u8,
        enforce_uniqueness: bool,
    ) -> FantasySeason {
        let teams = starting_teams.into_iter().map(Team::new).collect();

        let results = Vec::new();
        let status = Status::new();

        FantasySeason {
            name,
            teams,
            results,
            status,
            score_choice,
            draft_choice,
            lineup_size,
            season,
            grid_size,
            enforce_uniqueness,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn update_results(&mut self, race_results: RaceResults) -> Result<(), DownloadError> {
        let round = race_results.round;
        if self.status.has_results(round) {
            return Err(DownloadError::RaceResultsAlreadyDownloaded(round));
        }

        self.results.push(race_results);
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
                &self.score_choice,
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

    pub fn draft(&mut self, round: u8, df: &mut dyn Drafter) -> Result<(), DraftError> {
        if self.status.has_drafted(round) {
            return Err(DraftError::RoundLineupAlreadyExists(round));
        }

        let mut lineups = Vec::with_capacity(self.teams.len());
        for team in &self.teams {
            lineups.push(team.calculate_lineup(round, df)?);
        }

        if self.enforce_uniqueness {
            let mut already_seen = HashSet::new();
            for lineup in &lineups {
                for driver in lineup {
                    if already_seen.contains(&driver) {
                        return Err(DraftError::RoundDraftNonUnique(round, *driver));
                    }
                    already_seen.insert(driver);
                }
            }
        }

        for team in &mut self.teams {
            team.store_lineup(round, lineups.remove(0));
        }

        self.status.toggle_drafted(round);
        Ok(())
    }

    pub fn delete_round(&mut self, round: u8) -> Result<(), DeleteError> {
        if !self.status.has_results(round) {
            return Err(DeleteError::ResultsDeleteWhenResultsDontExist(round));
        }

        if self.status.has_scored(round) {
            self.teams.iter_mut().for_each(|t| t.delete_score(round));
            self.status.toggle_scored(round);
        }

        if self.status.has_results(round) {
            self.results.retain(|rr| rr.round != round);
            self.status.toggle_results(round);
        }

        Ok(())
    }

    pub fn delete_lineup(&mut self, round: u8) -> Result<(), DeleteError> {
        if self.status.has_scored(round) {
            return Err(DeleteError::LineupDeleteWhileScoresExist(round));
        }

        if self.status.has_drafted(round + 1) {
            return Err(DeleteError::LineupDeleteWhenNextRoundDrafted(round));
        }

        self.teams.iter_mut().for_each(|t| t.delete_round(round));
        if self.status.has_drafted(round) {
            self.status.toggle_drafted(round);
        }

        Ok(())
    }

    pub fn get_team_names(&self) -> Vec<String> {
        self.teams.iter().map(Team::name).collect()
    }

    pub fn get_season(&self) -> u16 {
        self.season
    }

    // returns an ordered list of (TeamName, Points) pairs, sorted first to last. Tiebreakers are,
    // as follows: most points, highest points round, highest lowest-points-round,order in fantasy_season.
    // as a result, teams in fantasy_season should be added in tiebreaking order, eg: the reverse
    // standings from the previous season
    pub fn get_points_by(&self, round: u8) -> Vec<(String, i16)> {
        let mut teams: Vec<_> = self.teams.iter().collect();
        teams.sort_by(|a, b| Team::sort_by(a, b, round));
        teams
            .into_iter()
            .map(|t| (t.name(), t.get_points_by(round)))
            .collect()
    }

    pub fn get_points_at(&self, round: u8) -> Option<Vec<(String, i16)>> {
        if self.status.has_scored(round) {
            let mut teams: Vec<_> = self.teams.iter().collect();
            teams.sort_by(|a, b| Team::sort_at(a, b, round));
            Some(
                teams
                    .into_iter()
                    .map(|t| {
                        (
                            t.name(),
                            t.get_points_at(round).expect("status out of date"),
                        )
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn get_lineup_at(&self, round: u8) -> HashMap<String, Vec<u8>> {
        let mut map = HashMap::new();
        self.teams.iter().for_each(|t| {
            if let Some(lineup) = t.get_lineup_at(round) {
                map.insert(t.name(), lineup);
            }
        });
        map
    }

    pub fn get_lineup_size(&self) -> u8 {
        self.lineup_size
    }

    pub fn get_status_at(&self, round: u8) -> (bool, bool, bool) {
        (
            self.status.has_drafted(round),
            self.status.has_results(round),
            self.status.has_scored(round),
        )
    }

    pub fn enforces_unique(&self) -> bool {
        self.enforce_uniqueness
    }
}
