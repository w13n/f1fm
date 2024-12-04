use crate::error::DraftError;

#[derive(Copy, Clone, Default)]
pub enum DraftChoice {
    #[default]
    Skip,
    RollOn,
    AllReplace,
}

pub trait Drafter {
    fn draft(&self, team_name: &str, previous_drivers: &Vec<u8>) -> Result<Vec<u8>, DraftError>;
}

#[derive(Default)]
pub struct Skip {}

impl Skip {
    pub(crate) fn new() -> Skip {
        Skip {}
    }
}

impl Drafter for Skip {
    fn draft(&self, _: &str, previous_drivers: &Vec<u8>) -> Result<Vec<u8>, DraftError> {
        Ok(previous_drivers.to_vec())
    }
}
