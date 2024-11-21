use super::draft::Drafter;
use super::score::Scorer;
use super::DriverResult;
use crate::error::{DraftError, ScoreError};
use std::cell::Cell;
use std::cmp::Ordering;

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
        scorer: &dyn Scorer,
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
            points += scorer.score(grid_size, driver_result)
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
        drafter: &dyn Drafter,
    ) -> Result<Vec<u8>, DraftError> {
        let prev_round_drivers = &self
            .rounds
            .iter()
            .rev()
            .find(|trr| trr.round == round - 1)
            .expect("status out of sync")
            .lineup;

        drafter.draft(&self.name, prev_round_drivers)
    }
    pub fn store_lineup(&mut self, round: u8, lineup: Vec<u8>) {
        if self.rounds.iter().any(|tr| tr.round == round) {
            panic!("cannot update lineup for a round that has already been scored");
        }
        self.rounds.push(TeamRound::new(round, lineup));
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_points_by(&self, round: u8) -> i16 {
        self.rounds
            .iter()
            .filter(|tr| tr.round <= round)
            .map(|tr| tr.points.get().unwrap_or_default())
            .sum::<i16>()
    }

    pub fn get_points_at(&self, round: u8) -> Option<i16> {
        self.rounds
            .iter()
            .find(|tr| tr.round == round)
            .and_then(|tr| tr.points.get())
    }

    pub fn get_lineup_at(&self, round: u8) -> Option<Vec<u8>> {
        self.rounds
            .iter()
            .find(|tr| tr.round == round)
            .map(|tr| tr.lineup.clone())
    }

    pub fn sort_by(a: &Team, b: &Team, round: u8) -> Ordering {
        let a_points = a.get_points_by(round);
        let b_points = b.get_points_by(round);
        match b_points.cmp(&a_points) {
            Ordering::Equal => {
                let val = {
                    |p: &Team| {
                        p.rounds
                            .iter()
                            .filter(|r| r.round <= round)
                            .map(|r| r.points.get().unwrap_or_default())
                            .max()
                            .unwrap_or_default()
                    }
                };
                let a_max = val(a);
                let b_max = val(b);
                match b_max.cmp(&a_max) {
                    Ordering::Equal => {
                        let min = {
                            |p: &Team| {
                                p.rounds
                                    .iter()
                                    .filter(|r| r.round <= round)
                                    .map(|r| r.points.get().unwrap_or_default())
                                    .min()
                                    .unwrap_or_default()
                            }
                        };
                        let a_min = min(a);
                        let b_min = min(b);
                        a_min.cmp(&b_min)
                    }
                    other => other,
                }
            }
            other => other,
        }
    }

    pub fn sort_at(a: &Team, b: &Team, round: u8) -> Ordering {
        let pts = {
            |p: &Team| {
                p.rounds
                    .iter()
                    .find(|tr| tr.round == round)
                    .expect("status out of sync")
                    .points
                    .get()
                    .expect("status out of sync")
            }
        };
        let a = pts(a);
        let b = pts(b);
        b.cmp(&a)
    }
}

/// the drivers that a given team has for the round given
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
