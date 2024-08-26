use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

struct FantasySeason {
    teams: Vec<Team>,
    results: Vec<RaceResult>,
    scorer: fn(&DriverResult) -> i16,
    drafter: fn(Vec<u8>) -> Vec<u8>,
    team_count: u16,
    driver_count: u8,
    season: u16,

}

impl FantasySeason {
    fn score(&mut self, round: u8) -> Result<(), ScoreError> {
        let mut team_results = Vec::with_capacity(self.team_count as usize);
        let race_result: RaceResult;

        if let Some(result) = self.results.iter().find(|result| result.is_round(round)) {
            race_result = result;
        } else {
            return Err(ScoreError::RoundResultsDoNotExist(round))
        }

        for team in self.teams {
            let maybe_team_result = team.get_team_race_result(round, self.scorer, &race_result.drivers);
            if let Ok(team_result) = maybe_team_result {
                team_results.push(team_result);
            } else {
                return maybe_team_result;
            }
        }
        for (i, result) in team_results.iter().enumerate() {
            self.teams.get(i).unwrap().update_points(result)
        }
    }
}

struct Team {
    name: String,
    drivers: Vec<TeamRaceLineup>, // the driver lineup of this team for each round
    points: Vec<TeamRaceResult>, // the points gained for this team per round
}

impl Team {

    // compute the TeamRaceResult for the round given for this team based on scorer and drivers
    // O(n) where n is the number of drivers on this team (assumes the total drivers in F1 remains at 20)
    fn get_team_race_result(&self, round: u8, scorer: fn(&DriverResult) -> i16, drivers: &Vec<DriverResult>) -> Result<TeamRaceResult, ScoreError> {
        if let Some(race_lineup) = self.drivers.iter().rev().find(|lineup| lineup.is_round(round)) {
            return race_lineup.get_lineup_result(scorer, drivers);
        }
        Err(ScoreError::RoundLineupDoesNotExist(round))
    }

    fn update_points(&mut self, team_race_result: TeamRaceResult) {
        self.points.push(team_race_result);
    }
}

// the drivers that a given team has for the round given
struct TeamRaceLineup {
    round: u8, // which round this lineup is for
    drivers: Vec<u8>, // which drivers are on this team for this round
}

impl TeamRaceLineup {

    // compute the TeamRaceResult for this teams driver lineup based on the DriverResults and scorer
    // O(n) where n is the number of drivers on this team (assumes the total drivers in F1 remains at 20)
    fn get_lineup_result(&self, scorer: fn(&DriverResult) -> i16, drivers: &Vec<DriverResult>) -> Result<TeamRaceResult, ScoreError> {
        let mut points: i16 = 0;
        for team_driver in self.drivers {
            if let Some(driver_result) = drivers.iter().find(|driver| driver.is_driver(team_driver)) {
                points += scorer(driver_result);
            } else {
                return Err(ScoreError::DriverDidNotRace(team_driver));
            }
        }
        Ok(TeamRaceResult{round: self.round, points})
    }

    fn is_round(&self, round: u8) -> bool {
        round == self.round
    }
}

// the points gained for this team from the round given
struct TeamRaceResult {
    round: u8, // which round these points were gained for
    points: i16, // the number of points gained for this round
}

// the results of a race for all drivers
struct RaceResult {
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

impl DriverResult {

    // computes if this DriverResult is for driver
    fn is_driver(&self, driver: u8) -> bool {
        driver == self.driver
    }
}

#[derive(Debug)]
enum ScoreError {
    DriverDidNotRace(u8),
    RoundLineupDoesNotExist(u8),
    RoundResultsDoNotExist(u8),
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

        }
    }
}

impl Error for ScoreError {}