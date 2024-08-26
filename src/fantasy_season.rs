use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

struct FantasySeason {
    teams: Vec<Team>,
    results: Vec<RaceResults>,
    status: Status,
    scorer: fn(&DriverResult) -> i16,
    drafter: fn(Vec<u8>) -> Vec<u8>,
    team_count: u16,
    driver_count: u8,
    season: u16,
}

impl FantasySeason {
    pub fn score(&mut self, round: u8) -> Result<(), ScoreError> {
        let mut team_results = Vec::with_capacity(self.team_count as usize);

        if !self.status.has_results(round) {
            return Err(ScoreError::RoundResultsDoNotExist(round));
        }

        if !self.status.has_drafted(round) {
            return Err(ScoreError::RoundLineupDoesNotExist(round));
        }

        if self.status.has_scored(round) {
            return Err(ScoreError::RoundAlreadyScored(round))
        }

        let race_result = self.results.iter().find(|rr| rr.round == round).expect("status out of sync");

        for team in &self.teams {
            let maybe_team_result = team.get_team_race_result(round, self.scorer, &race_result.drivers);
            if let Err(team_result) = maybe_team_result {
                return Err(team_result)
            }
            team_results.push(maybe_team_result.unwrap());
        }
        for team in &mut self.teams {
            team.update_points(team_results.remove(0));
        }

        self.status.toggle_scored(round);
        Ok(())
    }

    
}

struct Team {
    name: String,
    drivers: Vec<TeamRoundLineup>, // the driver lineup of this team for each round
    points: Vec<TeamRoundResult>, // the points gained for this team per round
}

impl Team {

    // compute the TeamRoundResult for the round given for this team based on scorer and drivers
    // O(n) where n is the number of drivers on this team (assumes the total drivers in F1 remains at 20)
    fn get_team_race_result(&self, round: u8, scorer: fn(&DriverResult) -> i16, drivers: &Vec<DriverResult>) -> Result<TeamRoundResult, ScoreError> {
        self.drivers
            .iter().rev().find(|trr| trr.round == round)
            .expect("status out of sync")
            .get_lineup_result(scorer, drivers)
    }

    // computes if this team already has a TeamRoundResult for the given round
    fn contains_results_for_round(&self, round: u8) -> bool {
        self.points.iter().find(|trr| trr.round == round).is_some()
    }

    // saves the new TeamRoundResult to this Team
    // Panics: if this Team already has a TeamRoundResult for the given TRR's round
    fn update_points(&mut self, team_round_result: TeamRoundResult) {
        if self.contains_results_for_round(team_round_result.round) {
            panic!("cannot not update points for a round that has already been scored");
        }
        self.points.push(team_round_result);
    }
}

// the drivers that a given team has for the round given
struct TeamRoundLineup {
    round: u8, // which round this lineup is for
    drivers: Vec<u8>, // which drivers are on this team for this round
}

impl TeamRoundLineup {

    // compute the TeamRoundResult for this teams driver lineup based on the DriverResults and scorer
    // O(n) where n is the number of drivers on this team (assumes the total drivers in F1 remains at 20)
    fn get_lineup_result(&self, scorer: fn(&DriverResult) -> i16, drivers: &Vec<DriverResult>) -> Result<TeamRoundResult, ScoreError> {
        let mut points: i16 = 0;
        for team_driver in &self.drivers {
            if let Some(driver_result) = drivers.iter().find(|dr| dr.driver == *team_driver) {
                points += scorer(driver_result);
            } else {
                return Err(ScoreError::DriverDidNotRace(*team_driver));
            }
        }
        Ok(TeamRoundResult {round: self.round, points})
    }
}

// the points gained for this team from the round given
struct TeamRoundResult {
    round: u8, // which round these points were gained for
    points: i16, // the number of points gained for this round
}

// the results of a race for all drivers
struct RaceResults {
    round: u8,
    drivers: Vec<DriverResult>,
}

// the results for a driver in a round
struct DriverResult {
    driver: u8,
    final_position: u8,
    grid_position: u8,
    qualifying_position: u8,
}

// stores the status of each round, if the race results have been collected, the teams have drafted, or the teams have been scored
struct Status {
    tasks: Vec<RoundStatus>, // a collection of statuses where the nth element of the RoundStatus is the nth  round
}

impl Status {

    fn _get_round_status(& self, round: u8) -> Option<&RoundStatus> {
        if self.tasks.len() >= round as usize {
            return self.tasks.get(round as usize - 1);
        }
        None
    }

    fn _find_mut_round_status(&mut self, round: u8) -> &mut RoundStatus {
        if self.tasks.len() < round as usize  {
            for _ in self.tasks.len()..(round as usize) {
                self.tasks.push(RoundStatus::new())
            }
        }
        return self.tasks.get_mut(round as usize - 1).unwrap()
    }

    fn has_results(& self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.results;
        }
        false
    }
    fn has_drafted(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.drafted;
        }
        false
    }

    fn has_scored(&self, round: u8) -> bool {
        if let Some(status) = self._get_round_status(round) {
            return status.scored;
        }
        false
    }

    fn toggle_results(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.results = !rs.results;
    }

    fn toggle_drafted(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.drafted = !rs.drafted;
    }

    fn toggle_scored(&mut self, round: u8) {
        let rs = self._find_mut_round_status(round);
        rs.scored = !rs.scored;
    }
}

#[derive(Default)]
struct RoundStatus {
    results: bool,
    drafted: bool,
    scored: bool,
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
#[derive(Debug)]
enum ScoreError {
    DriverDidNotRace(u8),
    RoundLineupDoesNotExist(u8),
    RoundResultsDoNotExist(u8),
    RoundAlreadyScored(u8)
}

impl Display for ScoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreError::DriverDidNotRace(driver) =>
                write!(f, "Error: Cannot complete scoring. Driver {} is on a team but did not race.", driver),
            ScoreError::RoundLineupDoesNotExist(round) =>
                write!(f, "Error: Cannot complete scoring. Lineup for {} round does not exist for at-least one team.", round),
            ScoreError::RoundResultsDoNotExist(round) =>
                write!(f, "Error: Cannot complete scoring. Race results for {} round do not exist for at-least one team.", round),
            ScoreError::RoundAlreadyScored(round) =>
                write!(f, "Error: Cannot complete scoring. Scores for {} round already exist for at-least one team.", round),

        }
    }
}

impl Error for ScoreError {}