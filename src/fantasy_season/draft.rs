use super::error::DraftError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq, Ord, Eq, Deserialize, Serialize)]
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
        write!(f, "{str}")
    }
}

pub trait Drafter: Debug + Sync + Send {
    fn draft(&mut self, team_name: &str, previous_drivers: &[u8]) -> Result<Vec<u8>, DraftError>;
}

#[derive(Default, Debug, Clone)]
pub struct Skip {}

impl Skip {
    pub(crate) fn new() -> Skip {
        Skip {}
    }
}

impl Drafter for Skip {
    fn draft(&mut self, _: &str, previous_drivers: &[u8]) -> Result<Vec<u8>, DraftError> {
        Ok(previous_drivers.to_vec())
    }
}

#[derive(Default, Debug, Clone)]
pub struct RollOn {
    drafted_drivers: HashMap<String, u8>,
}

impl RollOn {
    pub(crate) fn new(drafted_drivers: HashMap<String, u8>) -> RollOn {
        RollOn { drafted_drivers }
    }
}

impl Drafter for RollOn {
    fn draft(&mut self, team: &str, previous_drivers: &[u8]) -> Result<Vec<u8>, DraftError> {
        let team = team.to_string();
        if self.drafted_drivers.contains_key(&team) {
            let mut lineup = previous_drivers.to_vec();
            lineup.pop();
            lineup.insert(0, *self.drafted_drivers.get(&team).unwrap());
            Ok(lineup)
        } else {
            Err(DraftError::IncompleteDrafter)
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ReplaceAll {
    team_lineups: HashMap<String, Vec<u8>>,
}

impl ReplaceAll {
    pub(crate) fn new(team_lineups: HashMap<String, Vec<u8>>) -> ReplaceAll {
        ReplaceAll { team_lineups }
    }
}
impl Drafter for ReplaceAll {
    fn draft(&mut self, team: &str, _: &[u8]) -> Result<Vec<u8>, DraftError> {
        let team = team.to_string();
        if self.team_lineups.contains_key(&team) {
            Ok(self.team_lineups.remove(&team).unwrap())
        } else {
            Err(DraftError::IncompleteDrafter)
        }
    }
}
