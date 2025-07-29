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
use status::TeamStatus;
use std::collections::{HashMap, HashSet};
use team::Team;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FantasySeason {
    name: String,
    teams: Vec<Team>,
    results: HashMap<u8, RaceResults>,
    team_status: TeamStatus,
    score_choice: ScoreChoice,
    draft_choice: DraftChoice,
    lineup_size: u8,
    season: u16,
    grid_size: u8,
    enforce_uniqueness: bool,
}

impl FantasySeason {
    #[allow(clippy::too_many_arguments)]
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

        let results = HashMap::new();
        let status = TeamStatus::new();

        FantasySeason {
            name,
            teams,
            results,
            team_status: status,
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

    pub fn update_results(
        &mut self,
        round: u8,
        race_results: RaceResults,
    ) -> Result<(), DownloadError> {
        if self.results.contains_key(&round) {
            return Err(DownloadError::RaceResultsAlreadyDownloaded(round));
        }

        self.results.insert(round, race_results);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_score_choice(&self) -> ScoreChoice {
        self.score_choice
    }

    pub fn score(&mut self, round: u8) -> Result<(), ScoreError> {
        if !self.team_status.has_drafted(round) {
            return Err(ScoreError::RoundLineupDoesNotExist(round));
        }
        if self.team_status.has_scored(round) {
            return Err(ScoreError::RoundResultsAlreadyExist(round));
        }

        let driver_results = &self
            .results
            .get(&round)
            .ok_or(ScoreError::RoundResultsDoNotExist(round))?
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

        self.team_status.toggle_scored(round);
        Ok(())
    }

    pub fn get_draft_choice(&self) -> DraftChoice {
        self.draft_choice
    }

    pub fn draft(&mut self, round: u8, df: &mut dyn Drafter) -> Result<(), DraftError> {
        if self.team_status.has_drafted(round) {
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

        self.team_status.toggle_drafted(round);
        Ok(())
    }

    pub fn delete_round(&mut self, round: u8) -> Result<(), DeleteError> {
        if self.team_status.has_scored(round) {
            self.teams.iter_mut().for_each(|t| t.delete_score(round));
            self.team_status.toggle_scored(round);
        }

        let _ = self
            .results
            .remove(&round)
            .ok_or(DeleteError::ResultsDeleteWhenResultsDontExist(round));

        Ok(())
    }

    pub fn delete_lineup(&mut self, round: u8) -> Result<(), DeleteError> {
        if self.team_status.has_scored(round) {
            return Err(DeleteError::LineupDeleteWhileScoresExist(round));
        }

        if self.team_status.has_drafted(round + 1) {
            return Err(DeleteError::LineupDeleteWhenNextRoundDrafted(round));
        }

        self.teams.iter_mut().for_each(|t| t.delete_round(round));
        if self.team_status.has_drafted(round) {
            self.team_status.toggle_drafted(round);
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
        if self.team_status.has_scored(round) {
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
            self.team_status.has_drafted(round),
            self.results.contains_key(&round),
            self.team_status.has_scored(round),
        )
    }

    pub fn enforces_unique(&self) -> bool {
        self.enforce_uniqueness
    }
}
