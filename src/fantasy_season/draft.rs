use crate::error::DraftError;
use crate::fantasy_season::score::ScoreChoice;
use std::fmt::Display;

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum DraftChoice {
    #[default]
    Skip,
    RollOn,
    ReplaceAll,
}

impl Display for DraftChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DraftChoice::Skip => String::from("Skip"),
            DraftChoice::RollOn => String::from("Roll On"),
            DraftChoice::ReplaceAll => String::from("Replace All"),
        };
        write!(f, "{}", str)
    }
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
