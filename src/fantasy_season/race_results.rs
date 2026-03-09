use super::error::DownloadError;
use crate::api::Api;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

// the results of a race for all drivers
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RaceResults {
    pub(super) drivers: HashMap<u8, DriverResult>,
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

        let mut drivers = HashMap::new();
        for result in race_results {
            let driver = result.driver.permanent_number as u8;
            let final_position = result.position as u8;
            let grid_position = result.grid as u8;
            let qualifying_position = qualifying_results
                .iter()
                .find(|qr| qr.driver.permanent_number as u8 == driver)
                .map(|x| x.position as u8)
                .unwrap_or(grid_position);
            drivers.insert(
                driver,
                DriverResult::new(final_position, grid_position, qualifying_position),
            );
        }

        Ok(RaceResults { drivers })
    }
}

// the results for a driver in a round
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub(super) struct DriverResult {
    pub final_position: u8,
    pub grid_position: u8,
    pub qualifying_position: u8,
}

impl DriverResult {
    fn new(final_pos: u8, grid_pos: u8, qualifying_pos: u8) -> DriverResult {
        DriverResult {
            final_position: final_pos,
            grid_position: grid_pos,
            qualifying_position: qualifying_pos,
        }
    }
}
