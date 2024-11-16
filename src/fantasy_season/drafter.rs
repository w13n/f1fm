use crate::error::{DraftError, ResultError};

pub enum DraftChoice {
    Skip,
}

pub trait Drafter {
    fn draft(&self, team_name: &str, previous_drivers: &Vec<u8>) -> Result<Vec<u8>, DraftError>;
}

pub struct Skipper {}

impl Drafter for Skipper {
    fn draft(&self, _: &str, previous_drivers: &Vec<u8>) -> Result<Vec<u8>, DraftError> {
        Ok(previous_drivers.to_vec())
    }
}
