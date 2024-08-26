use std::cmp::Ordering;
use ergast_rs::apis::race_table::{QualifyingResult, RaceResult};
use ergast_rs::apis::response::Response;
use reqwest::blocking::Client;
use crate::error::ResultError;

// the results of a race for all drivers
#[derive(Debug)]
pub struct RaceResults {
    pub round: u8,
    pub drivers: Vec<DriverResult>,
}

impl RaceResults {
    pub(crate) fn build(round: u8, season: u16) -> Result<RaceResults, ResultError> {
        let api_client = Client::new();
        let qualifying_response = api_client
            .get(format!("https://api.jolpi.ca/ergast/f1/{season}/{round}/qualifying/"))
            .send().map_err(|_| ResultError::CannotConnectToServer)?
            .json::<Response>().map_err(|_| ResultError::CannotParseJson(round))?;

        let rt = qualifying_response.data.race_table.unwrap();
        let race = rt.races.get(0).ok_or(ResultError::RaceResultsNotYetAvailable(round))?;

        let mut qualifying_pairings: Vec<QualifyingPairing> = Vec::new();
        for result in race.qualifying_results.as_ref().unwrap() {
            qualifying_pairings.push(QualifyingPairing::new(result))
        }

        let race_response = api_client
            .get(format!("https://api.jolpi.ca/ergast/f1/{season}/{round}/results/"))
            .send().map_err(|_| ResultError::CannotConnectToServer)?
            .json::<Response>().map_err(|_| ResultError::CannotParseJson(round))?;

        let rt = race_response.data.race_table.unwrap();
        let race = rt.races.get(0).ok_or(ResultError::RaceResultsNotYetAvailable(round))?;

        let mut broken_drivers: Vec<BrokenDriverResult> = Vec::with_capacity(qualifying_pairings.len());
        for result in race.race_results.as_ref().unwrap() {
            broken_drivers.push(BrokenDriverResult::new(result, &qualifying_pairings))
        }

        broken_drivers.sort();

        let mut drivers: Vec<DriverResult> = Vec::with_capacity(broken_drivers.len());

        for (i, driver) in broken_drivers.iter().enumerate() {
            drivers.push(DriverResult::new(driver, (i as u8 + 1)))
        }

        Ok( RaceResults{round, drivers})
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
            qualifying_position: broken.qualifying_position
        }
    }
}

struct QualifyingPairing {
    driver: u8,
    qualifying_position: u8,
}

impl QualifyingPairing {
    fn new(result: &QualifyingResult) -> QualifyingPairing {
        let driver = result.driver.permanent_number as u8;
        let qualifying_position = result.position as u8;
        QualifyingPairing {driver, qualifying_position}
    }
}

struct BrokenDriverResult {
    driver: u8,
    final_position: u8,
    grid_position: u8,
    qualifying_position: u8,
}

impl BrokenDriverResult {
    fn new(result: &RaceResult, pairings: &Vec<QualifyingPairing>) -> BrokenDriverResult {
        let driver = result.driver.permanent_number as u8;
        let final_position = result.position as u8;
        let grid_position = result.grid as u8;
        let pairing = pairings.iter().find(|qp| qp.driver == driver).expect("qualifying does not match race");
        let qualifying_position = pairing.qualifying_position;

        BrokenDriverResult {driver, final_position, grid_position, qualifying_position}
    }
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
        return Some(self.cmp(other));
    }
}

impl Ord for BrokenDriverResult {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.grid_position == other.grid_position {
            return self.qualifying_position.cmp(&other.qualifying_position)
        } else if self.grid_position == 0 {
            Ordering::Greater
        } else if other.grid_position == 0 {
            Ordering::Less
        } else {
            self.grid_position.cmp(&other.grid_position)
        }
    }
}