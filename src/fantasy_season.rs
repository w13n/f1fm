mod team;
pub mod race_results;

use std::fmt::{Debug, Display};
use crate::error::{ScoreError, DraftError};
use super::fantasy_season::race_results::*;
use super::fantasy_season::team::Team;

pub struct FantasySeason {
    teams: Vec<Team>,
    results: Vec<RaceResults>,
    status: Status,
    scorer: fn(&DriverResult) -> i16,
    drafter: fn(&str, &Vec<u8>) -> Result<Vec<u8>, DraftError>,
    team_count: u16,
    driver_count: u8,
    season: u16,
    enforce_uniqueness: bool,
}

impl FantasySeason {
    pub fn score(&mut self, round: u8) -> Result<(), ScoreError> {
        let mut team_results = Vec::with_capacity(self.team_count as usize);

        if !self.status.has_results(round) {
            return Err(ScoreError::RoundResultsDoNotExist(round));
        }

        if !self.status.has_drafted(round) {
            return Err(ScoreError::RoundLineupDoesNotExist(round));
        }

        if self.status.has_scored(round) {
            return Err(ScoreError::RoundResultsAlreadyExist(round))
        }

        let race_result = self.results.iter().find(|rr| rr.round == round).expect("status out of sync");

        for team in &self.teams {
            let maybe_team_result = team.get_team_race_result(round, self.scorer, &race_result.drivers);
            if let Err(error) = maybe_team_result {
                return Err(error)
            }
            team_results.push(maybe_team_result.unwrap());
        }
        for team in &mut self.teams {
            team.update_points(team_results.remove(0));
        }

        self.status.toggle_scored(round);
        Ok(())
    }

    pub fn draft(&mut self, round: u8) -> Result<(), DraftError> {
        let mut team_round_lineups = Vec::with_capacity(self.team_count as usize);


        if !self.status.has_drafted(round - 1) {
            return Err(DraftError::PreviousRoundLineupDoesNotExist(round - 1))
        }

        if self.status.has_drafted(round) {
            return Err(DraftError::RoundLineupAlreadyExists(round));
        }

        for team in &mut self.teams {
            let maybe_team_round_lineup = team.get_team_lineup(round, self.drafter);
            if let Err(error) = maybe_team_round_lineup {
                return Err(error);
            }
            team_round_lineups.push(maybe_team_round_lineup.unwrap());

        }

        if (self.enforce_uniqueness) {
            //TODO
        }

        for team in &mut self.teams {
            team.update_lineup(team_round_lineups.remove(0));
        }

        self.status.toggle_drafted(round);
        Ok(())
    }

    
}

// stores the status of each round, if the race results have been collected, the teams have drafted, or the teams have been scored
struct Status {
    tasks: Vec<RoundStatus>, // a collection of statuses where the nth element of the RoundStatus is the nth  round
}

impl Status {

    fn _get_round_status(& self, round: u8) -> Option<&RoundStatus> {
        if self.tasks.len() >= round as usize {
            return self.tasks.get(round as usize - 1);
        }
        None
    }

    fn _find_mut_round_status(&mut self, round: u8) -> &mut RoundStatus {
        if self.tasks.len() < round as usize  {
            for _ in self.tasks.len()..(round as usize) {
                self.tasks.push(RoundStatus::new())
            }
        }
        return self.tasks.get_mut(round as usize - 1).unwrap()
    }

    fn has_results(& self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.results;
        }
        false
    }
    fn has_drafted(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.drafted;
        }
        false
    }

    fn has_scored(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.scored;
        }
        false
    }

    fn toggle_results(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.results = !rs.results;
    }

    fn toggle_drafted(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.drafted = !rs.drafted;
    }

    fn toggle_scored(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.scored = !rs.scored;
    }
}

#[derive(Default)]
struct RoundStatus {
    results: bool,
    drafted: bool,
    scored: bool,
}

impl RoundStatus {
    fn new() -> RoundStatus {
        RoundStatus {
            results: false,
            drafted: false,
            scored: false,
        }
    }
}