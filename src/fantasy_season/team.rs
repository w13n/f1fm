use super::DriverResult;
use crate::error::{DraftError, ScoreError};
use std::cell::Cell;

pub struct Team {
    name: String,
    rounds: Vec<TeamRound>, // the driver lineup and points scored of this team for each round
}

impl Team {
    pub fn new(name: String, r1_lineup: Vec<u8>) -> Team {
        Team {
            name,
            rounds: vec![TeamRound::new(1, r1_lineup)],
        }
    }

    pub fn calculate_score(
        &self,
        round: u8,
        grid_size: u8,
        scorer: fn(u8, &DriverResult) -> i16,
        driver_results: &[DriverResult],
    ) -> Result<i16, ScoreError> {
        let team_round = self
            .rounds
            .iter()
            .rev()
            .find(|tr| tr.round == round)
            .expect("status out of date: scoring");

        let mut points = 0;

        for driver in &team_round.lineup {
            let driver_result = driver_results
                .iter()
                .find(|dr| dr.driver == *driver)
                .ok_or(ScoreError::DriverDidNotRace(*driver))?;
            points += scorer(grid_size, driver_result)
        }
        Ok(points)
    }

    pub fn store_score(&mut self, round: u8, score: i16) {
        let team_round = self
            .rounds
            .iter()
            .rev()
            .find(|tr| tr.round == round)
            .expect("status out of date: scoring");

        team_round.points.set(Some(score));
    }

    pub fn calculate_lineup(
        &self,
        round: u8,
        drafter: fn(&str, &Vec<u8>) -> Result<Vec<u8>, DraftError>,
    ) -> Result<Vec<u8>, DraftError> {
        let prev_round_drivers = &self
            .rounds
            .iter()
            .rev()
            .find(|trr| trr.round == round - 1)
            .expect("status out of sync")
            .lineup;

        drafter(&self.name, prev_round_drivers)
    }
    pub fn store_lineup(&mut self, round: u8, lineup: Vec<u8>) {
        if self.rounds.iter().any(|tr| tr.round == round) {
            panic!("cannot update lineup for a round that has already been scored");
        }
        self.rounds.push(TeamRound::new(round, lineup));
    }
}

// the drivers that a given team has for the round given
pub struct TeamRound {
    round: u8,                 // which round this lineup is for
    lineup: Vec<u8>,           // which drivers are on this team for this round
    points: Cell<Option<i16>>, // the number of points gained for this round
}

impl TeamRound {
    pub fn new(round: u8, lineup: Vec<u8>) -> TeamRound {
        TeamRound {
            round,
            lineup,
            points: Cell::new(None),
        }
    }
}
