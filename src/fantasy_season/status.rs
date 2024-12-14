// stores if the race results have been collected, the teams have drafted, or the teams have been scored for each round
#[derive(Debug)]
pub(super) struct Status {
    tasks: Vec<RoundStatus>, // a collection of statuses where the nth element of the RoundStatus is the nth  round
}

impl Status {
    pub fn new() -> Status {
        Status { tasks: Vec::new() }
    }
    fn _get_round_status(&self, round: u8) -> Option<&RoundStatus> {
        if self.tasks.len() >= round as usize {
            return self.tasks.get(round as usize - 1);
        }
        None
    }

    fn _find_mut_round_status(&mut self, round: u8) -> &mut RoundStatus {
        if self.tasks.len() < round as usize {
            for _ in self.tasks.len()..(round as usize) {
                self.tasks.push(RoundStatus::new())
            }
        }
        return self.tasks.get_mut(round as usize - 1).unwrap();
    }

    pub fn has_results(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.results;
        }
        false
    }
    pub fn has_drafted(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.drafted;
        }
        false
    }

    pub fn has_scored(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.scored;
        }
        false
    }

    pub fn toggle_results(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.results = !rs.results;
    }

    pub fn toggle_drafted(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.drafted = !rs.drafted;
    }

    pub fn toggle_scored(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.scored = !rs.scored;
    }
}

#[derive(Default, Debug)]
struct RoundStatus {
    pub results: bool,
    pub drafted: bool,
    pub scored: bool,
}

impl RoundStatus {
    fn new() -> RoundStatus {
        RoundStatus {
            results: false,
            drafted: false,
            scored: false,
        }
    }
}
