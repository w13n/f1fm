use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// stores if the race results have been collected, the teams have drafted, or the teams have been scored for each round
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct TeamStatus(HashMap<u8, RoundStatus>);

impl TeamStatus {
    pub fn new() -> TeamStatus {
        TeamStatus(HashMap::new())
    }
    pub fn has_drafted(&self, round: u8) -> bool {
        self.0.get(&round).map(|x| x.drafted).unwrap_or(false)
    }

    pub fn has_scored(&self, round: u8) -> bool {
        self.0.get(&round).map(|x| x.scored).unwrap_or(false)
    }

    pub fn toggle_drafted(&mut self, round: u8) {
        if let Some(x) = self.0.get_mut(&round) {
            x.drafted = !x.drafted;
        } else {
            self.0.insert(round, RoundStatus::new(true, false));
        }
    }

    pub fn toggle_scored(&mut self, round: u8) {
        if let Some(x) = self.0.get_mut(&round) {
            x.scored = !x.scored;
        } else {
            self.0.insert(round, RoundStatus::new(false, true));
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct RoundStatus {
    pub drafted: bool,
    pub scored: bool,
}

impl RoundStatus {
    fn new(drafted: bool, scored: bool) -> RoundStatus {
        RoundStatus { drafted, scored }
    }
}
