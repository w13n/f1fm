use super::DriverResult;
use super::draft::Drafter;
use super::score::Scorer;
use crate::error::{DraftError, ScoreError};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Team {
    name: String,
    rounds: HashMap<u8, TeamRound>, // the driver lineup and points scored of this team for each round
}

impl Team {
    pub fn new(name: String, r1_lineup: Vec<u8>) -> Team {
        let mut rounds = HashMap::new();
        rounds.insert(1, TeamRound::new(r1_lineup));
        Team { name, rounds }
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
            .get(&round)
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
            .get(&round)
            .expect("status out of date: scoring");

        team_round.points.set(Some(score));
    }

    pub fn delete_score(&mut self, round: u8) {
        self.rounds
            .get(&round)
            .expect("status out of date: deleting")
            .points
            .set(None);
    }

    pub fn delete_round(&mut self, round: u8) {
        self.rounds.remove(&round);
    }

    pub fn calculate_lineup(
        &self,
        round: u8,
        drafter: &mut dyn Drafter,
    ) -> Result<Vec<u8>, DraftError> {
        let prev_round_drivers = &self
            .rounds
            .get(&(round - 1))
            .expect("status out of sync")
            .lineup;

        drafter.draft(&self.name, prev_round_drivers)
    }
    pub fn store_lineup(&mut self, round: u8, lineup: Vec<u8>) {
        if self.rounds.contains_key(&round) {
            panic!("cannot update lineup for a round that has already been scored");
        }
        self.rounds.insert(round, TeamRound::new(lineup));
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_points_by(&self, round: u8) -> i16 {
        self.rounds
            .iter()
            .filter(|pair| pair.0 <= &round)
            .map(|pair| pair.1.points.get().unwrap_or_default())
            .sum::<i16>()
    }

    pub fn get_points_at(&self, round: u8) -> Option<i16> {
        self.rounds.get(&round).and_then(|r| r.points.get())
    }

    pub fn get_lineup_at(&self, round: u8) -> Option<Vec<u8>> {
        self.rounds.get(&round).map(|r| r.lineup.clone())
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
                            .filter(|pair| pair.0 <= &round)
                            .map(|pair| pair.1.points.get().unwrap_or_default())
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
                                    .filter(|pair| pair.0 <= &round)
                                    .map(|pair| pair.1.points.get().unwrap_or_default())
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
                    .get(&round)
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
#[derive(Debug, Deserialize, Serialize)]
pub struct TeamRound {
    lineup: Vec<u8>,           // which drivers are on this team for this round
    points: Cell<Option<i16>>, // the number of points gained for this round
}

impl TeamRound {
    pub fn new(lineup: Vec<u8>) -> TeamRound {
        TeamRound {
            lineup,
            points: Cell::new(None),
        }
    }
}
