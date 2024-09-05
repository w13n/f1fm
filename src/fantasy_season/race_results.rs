use crate::error::ResultError;
use ergast_rs::apis::response::Response;
use reqwest::blocking::Client;
use std::cmp::Ordering;

// the results of a race for all drivers
#[derive(Debug)]
pub struct RaceResults {
    pub round: u8,
    pub drivers: Vec<DriverResult>,
}

impl RaceResults {
    pub fn build(round: u8, season: u16) -> Result<RaceResults, ResultError> {
        let api_client = Client::new();
        let mut races = api_client
            .get(format!(
                "https://api.jolpi.ca/ergast/f1/{season}/{round}/qualifying/"
            ))
            .send()
            .map_err(|_| ResultError::CannotConnectToServer)?
            .json::<Response>()
            .map_err(|_| ResultError::CannotParseJson(round))?
            .data
            .race_table
            .expect("bad response")
            .races;
        let qualifying_results = if !races.is_empty() {
            races
                .swap_remove(0)
                .qualifying_results
                .expect("bad response")
        } else {
            return Err(ResultError::RaceResultsNotYetAvailable(round));
        };
        races = api_client
            .get(format!(
                "https://api.jolpi.ca/ergast/f1/{season}/{round}/results/"
            ))
            .send()
            .map_err(|_| ResultError::CannotConnectToServer)?
            .json::<Response>()
            .map_err(|_| ResultError::CannotParseJson(round))?
            .data
            .race_table
            .unwrap()
            .races;
        let race_results = if !races.is_empty() {
            races.swap_remove(0).race_results.unwrap()
        } else {
            return Err(ResultError::RaceResultsNotYetAvailable(round));
        };

        let mut broken_drivers = Vec::new();
        for result in race_results {
            let driver = result.driver.permanent_number as u8;
            let final_position = result.position as u8;
            let grid_position = result.grid as u8;
            let qualifying_position = qualifying_results
                .iter()
                .find(|qr| qr.driver.permanent_number as u8 == driver)
                .expect("driver raced but not qualified")
                .position as u8;
            broken_drivers.push(BrokenDriverResult {
                driver,
                final_position,
                grid_position,
                qualifying_position,
            })
        }
        broken_drivers.sort();
        let drivers = broken_drivers
            .iter()
            .enumerate()
            .map(|(pos, bd)| DriverResult::new(bd, pos as u8 + 1))
            .collect();

        Ok(RaceResults { round, drivers })
    }
}

// the results for a driver in a round
#[derive(Debug)]

pub struct DriverResult {
    pub driver: u8,
    pub final_position: u8,
    pub grid_position: u8,
    pub qualifying_position: u8,
}

impl DriverResult {
    fn new(broken: &BrokenDriverResult, pos: u8) -> DriverResult {
        DriverResult {
            driver: broken.driver,
            final_position: broken.final_position,
            grid_position: pos,
            qualifying_position: broken.qualifying_position,
        }
    }
}
struct BrokenDriverResult {
    driver: u8,
    final_position: u8,
    grid_position: u8,
    qualifying_position: u8,
}

impl Eq for BrokenDriverResult {}

impl PartialEq<Self> for BrokenDriverResult {
    fn eq(&self, other: &Self) -> bool {
        self.grid_position == other.grid_position
            && self.qualifying_position == other.qualifying_position
    }
}

impl PartialOrd<Self> for BrokenDriverResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BrokenDriverResult {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.grid_position == other.grid_position {
            self.qualifying_position.cmp(&other.qualifying_position)
        } else if self.grid_position == 0 {
            Ordering::Greater
        } else if other.grid_position == 0 {
            Ordering::Less
        } else {
            self.grid_position.cmp(&other.grid_position)
        }
    }
}
