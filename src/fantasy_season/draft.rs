use crate::error::DraftError;

#[derive(Copy, Clone)]
pub enum DraftChoice {
    Skip,
}

pub trait Drafter {
    fn draft(&self, team_name: &str, previous_drivers: &Vec<u8>) -> Result<Vec<u8>, DraftError>;
}

#[derive(Default)]
pub struct Skipper {}

impl Skipper {
    pub(crate) fn new() -> Skipper {
        Skipper{}
    }
}

impl Drafter for Skipper {
    fn draft(&self, _: &str, previous_drivers: &Vec<u8>) -> Result<Vec<u8>, DraftError> {
        Ok(previous_drivers.to_vec())
    }
}
