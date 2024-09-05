use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ScoreError {
    DriverDidNotRace(u8),
    RoundLineupDoesNotExist(u8),
    RoundResultsDoNotExist(u8),
    RoundResultsAlreadyExist(u8),
}

impl Display for ScoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreError::DriverDidNotRace(driver) => {
                write!(f, "driver {} is on a team but did not race", driver)
            }
            ScoreError::RoundLineupDoesNotExist(round) => {
                write!(f, "lineup for round {} does not exist", round)
            }
            ScoreError::RoundResultsDoNotExist(round) => {
                write!(f, "race results for round {} does not exist", round)
            }
            ScoreError::RoundResultsAlreadyExist(round) => {
                write!(f, "scores for {} round already exists", round)
            }
        }
    }
}

impl Error for ScoreError {}

#[derive(Debug)]
pub enum DraftError {
    RoundLineupAlreadyExists(u8),
    PreviousRoundLineupDoesNotExist(u8),
    RoundDraftNonUnique(u8, u8),
}

impl Display for DraftError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DraftError::RoundLineupAlreadyExists(round) => {
                write!(f, "lineup for round {} already exist", round)
            }
            DraftError::PreviousRoundLineupDoesNotExist(prev_round) => write!(
                f,
                "lineup for the previous round ({})  does not exist",
                prev_round
            ),
            DraftError::RoundDraftNonUnique(round, driver) => write!(
                f,
                "lineup for round {} has multiple driver #{}",
                round, driver
            ),
        }
    }
}

impl Error for DraftError {}

#[derive(Debug)]
pub enum ResultError {
    CannotConnectToServer,
    CannotParseJson(u8),
    RaceResultsNotYetAvailable(u8),
    RaceResultsAlreadyDownloaded(u8),
}

impl Display for ResultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultError::CannotConnectToServer => write!(f, "cannot connect to server"),
            ResultError::CannotParseJson(round) => write!(
                f,
                "results for round ({}) could not be parsed and may not exist",
                round
            ),
            ResultError::RaceResultsNotYetAvailable(round) => {
                write!(f, "results for round ({}) are not yet available", round)
            }
            ResultError::RaceResultsAlreadyDownloaded(round) => write!(
                f,
                "results for round ({}) have already been downloaded",
                round
            ),
        }
    }
}

impl Error for ResultError {}
