use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ScoreError {
    DriverDidNotRace(u8),
    RoundLineupDoesNotExist(u8),
    RoundResultsDoNotExist(u8),
    RoundResultsAlreadyExist(u8)
}

impl Display for ScoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreError::DriverDidNotRace(driver) =>
                write!(f, "Error: Cannot complete scoring. Driver {} is on a team but did not race.", driver),
            ScoreError::RoundLineupDoesNotExist(round) =>
                write!(f, "Error: Cannot complete scoring. Lineup for round {} does not exist.", round),
            ScoreError::RoundResultsDoNotExist(round) =>
                write!(f, "Error: Cannot complete scoring. Race results for round {} does not exist.", round),
            ScoreError::RoundResultsAlreadyExist(round) =>
                write!(f, "Error: Cannot complete scoring. Scores for {} round already exists.", round),

        }
    }
}

impl Error for ScoreError {}

#[derive(Debug)]
pub enum DraftError {
    RoundLineupAlreadyExists(u8),
    PreviousRoundLineupDoesNotExist(u8),
}

impl Display for DraftError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DraftError::RoundLineupAlreadyExists(round) =>
                write!(f, "Error: Cannot complete drafting. Lineup for round {} already exist.", round),

            DraftError::PreviousRoundLineupDoesNotExist(prev_round) =>
                write!(f, "Error: Cannot complete drafting. Lineup for the previous round ({})  does not exist.", prev_round),

        }
    }
}

impl Error for DraftError {}

#[derive(Debug)]
pub enum ResultError {
    CannotConnectToServer,
    CannotParseJson(u8),
    RaceResultsNotYetAvailable(u8),
}

impl Display for ResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultError::CannotConnectToServer =>
                write!(f, "Error: Cannot download race results. Cannot connect to server."),
            ResultError::CannotParseJson(round) =>
                write!(f, "Error: Cannot download race results. Results for round ({}) could not be parsed and may not exist.", round),
            ResultError::RaceResultsNotYetAvailable(round) =>
                write!(f, "Error: Cannot download race results. Results for round ({}) are not yet available.", round),

        }
    }
}

impl Error for ResultError {}