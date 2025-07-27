use super::error::DownloadError;
use crate::vc::api::Api;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// the results of a race for all drivers
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RaceResults {
    pub(super) round: u8,
    pub(super) drivers: Vec<DriverResult>,
}

impl RaceResults {
    pub async fn build(round: u8, season: u16) -> Result<RaceResults, DownloadError> {
        let api_client = Api::new();

        let (qualifying_results_down, race_results_down) = tokio::join!(
            api_client.get_qualifying_results(season, round),
            api_client.get_race_results(season, round)
        );

        let qualifying_results = qualifying_results_down.map_err(DownloadError::ApiError)?;

        let race_results = race_results_down.map_err(DownloadError::ApiError)?;

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
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub(super) struct DriverResult {
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
