use crate::error::ApiError;
use ergast_rs::apis::race_table::{QualifyingResult, Race, RaceResult};
use ergast_rs::apis::response::Response;
use reqwest::blocking::Client;
use std::collections::HashMap;

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Api {
        Api {
            client: Client::new(),
        }
    }

    pub fn get_race_names(&self, season: u16) -> Result<HashMap<u8, String>, ApiError> {
        let mut map = HashMap::new();
        self.client
            .get(format!("https://api.jolpi.ca/ergast/f1/{season}/races/"))
            .send()
            .map_err(|_| ApiError::CannotConnectToServer)?
            .json::<Response>()
            .map_err(|_| ApiError::CannotParseJsonOther)?
            .data
            .race_table
            .expect("bad response")
            .races
            .into_iter()
            .for_each(|r| {
                map.insert(r.round as u8, r.name);
            });
        Ok(map)
    }

    pub fn get_race_results(&self, season: u16, round: u8) -> Result<Vec<RaceResult>, ApiError> {
        let mut races = self.get_races(season, round, "results")?;
        if !races.is_empty() {
            Ok(races.swap_remove(0).race_results.expect("bad response"))
        } else {
            Err(ApiError::RaceResultsNotYetAvailable(round))
        }
    }
    pub fn get_qualifying_results(
        &self,
        season: u16,
        round: u8,
    ) -> Result<Vec<QualifyingResult>, ApiError> {
        let mut races = self.get_races(season, round, "qualifying")?;
        if !races.is_empty() {
            Ok(races
                .swap_remove(0)
                .qualifying_results
                .expect("bad response"))
        } else {
            Err(ApiError::RaceResultsNotYetAvailable(round))
        }
    }

    fn get_races(&self, season: u16, round: u8, result_type: &str) -> Result<Vec<Race>, ApiError> {
        Ok(self
            .client
            .get(format!(
                "https://api.jolpi.ca/ergast/f1/{season}/{round}/{result_type}"
            ))
            .send()
            .map_err(|_| ApiError::CannotConnectToServer)?
            .json::<Response>()
            .map_err(|_| ApiError::CannotParseJsonRound(round))?
            .data
            .race_table
            .expect("bad response")
            .races)
    }
}
