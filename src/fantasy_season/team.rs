use crate::error::{DraftError, ScoreError};
use crate::fantasy_season::DriverResult;

pub struct Team {
    name: String,
    drivers: Vec<TeamRoundLineup>, // the driver lineup of this team for each round
    points: Vec<TeamRoundResult>, // the points gained for this team per round
}

impl Team {

    // compute the TeamRoundResult for the round given for this team based on scorer and drivers
    // O(n) where n is the number of drivers on this team (assumes the total drivers in F1 remains at 20)
    pub fn get_team_race_result(&self, round: u8, scorer: fn(&DriverResult) -> i16, drivers: &Vec<DriverResult>) -> Result<TeamRoundResult, ScoreError> {
        self.drivers
            .iter().rev().find(|trr| trr.round == round)
            .expect("status out of sync")
            .get_lineup_result(scorer, drivers)
    }

    // computes if this team already has a TeamRoundResult for the given round
    pub fn contains_results_for_round(&self, round: u8) -> bool {
        self.points.iter().find(|trr| trr.round == round).is_some()
    }

    // saves the new TeamRoundResult to this Team
    // Panics: if this Team already has a TeamRoundResult for the given TRR's round
    pub fn update_points(&mut self, team_round_result: TeamRoundResult) {
        if self.contains_results_for_round(team_round_result.round) {
            panic!("cannot not update points for a round that has already been scored");
        }
        self.points.push(team_round_result);
    }

    pub fn get_team_lineup(&mut self, round: u8, drafter: fn(&str, &Vec<u8>) -> Result<Vec<u8>, DraftError>) -> Result<TeamRoundLineup, DraftError> {
        let prev_round_drivers = &self.drivers
            .iter().rev().find(|trr| trr.round == round - 1)
            .expect("status out of sync")
            .drivers;

        let maybe_team_lineup = drafter(&*self.name, prev_round_drivers);
        if let Err(error) = maybe_team_lineup {
            return Err(error);
        }
        Ok(TeamRoundLineup{round, drivers: maybe_team_lineup.unwrap()})
    }

    pub fn update_lineup(&mut self, team_round_lineup: TeamRoundLineup) {
        if self.contains_lineup_for_round(team_round_lineup.round) {
            panic!("cannot update lineup for a round that has already been scored");
        }
        self.drivers.push(team_round_lineup);
    }

    fn contains_lineup_for_round(&self, round: u8) -> bool {
        self.drivers.iter().find(|trl| trl.round == round).is_some()
    }
}

// the drivers that a given team has for the round given
pub struct TeamRoundLineup {
    round: u8, // which round this lineup is for
    drivers: Vec<u8>, // which drivers are on this team for this round
}

impl TeamRoundLineup {

    // compute the TeamRoundResult for this teams driver lineup based on the DriverResults and scorer
    // O(n) where n is the number of drivers on this team (assumes the total drivers in F1 remains at 20)
    fn get_lineup_result(&self, scorer: fn(&DriverResult) -> i16, drivers: &Vec<DriverResult>) -> Result<TeamRoundResult, ScoreError> {
        let mut points: i16 = 0;
        for team_driver in &self.drivers {
            if let Some(driver_result) = drivers.iter().find(|dr| dr.driver == *team_driver) {
                points += scorer(driver_result);
            } else {
                return Err(ScoreError::DriverDidNotRace(*team_driver));
            }
        }
        Ok(TeamRoundResult {round: self.round, points})
    }
}

// the points gained for this team from the round given
pub struct TeamRoundResult {
    round: u8, // which round these points were gained for
    points: i16, // the number of points gained for this round
}