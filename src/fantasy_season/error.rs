use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone)]
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
                write!(f, "driver {driver} is on a team but did not race")
            }
            ScoreError::RoundLineupDoesNotExist(round) => {
                write!(f, "lineup for round {round} does not exist")
            }
            ScoreError::RoundResultsDoNotExist(round) => {
                write!(f, "race results for round {round} does not exist")
            }
            ScoreError::RoundResultsAlreadyExist(round) => {
                write!(f, "scores for {round} round already exists")
            }
        }
    }
}

impl Error for ScoreError {}

#[derive(Debug, Copy, Clone)]
pub enum DraftError {
    RoundLineupAlreadyExists(u8),
    PreviousRoundLineupDoesNotExist,
    RoundDraftNonUnique(u8, u8),
    IncompleteDrafter,
}

impl Display for DraftError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DraftError::RoundLineupAlreadyExists(round) => {
                write!(f, "lineup for round {round} already exist")
            }
            DraftError::PreviousRoundLineupDoesNotExist => {
                write!(f, "lineup for the previous round does not exist",)
            }
            DraftError::RoundDraftNonUnique(round, driver) => {
                write!(f, "lineup for round {round} has multiple drivers #{driver}")
            }
            DraftError::IncompleteDrafter => {
                write!(f, "drafter was constructed with incomplete information")
            }
        }
    }
}

impl Error for DraftError {}

#[derive(Debug, Copy, Clone)]
pub enum DownloadError {
    ApiError(ApiError),
    RaceResultsAlreadyDownloaded(u8),
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::ApiError(ae) => Display::fmt(&ae, f),
            DownloadError::RaceResultsAlreadyDownloaded(round) => write!(
                f,
                "results for round {} have already been downloaded",
                round
            ),
        }
    }
}

impl Error for DownloadError {}

#[derive(Debug, Copy, Clone)]
pub enum ApiError {
    CannotConnectToServer,
    CannotParseJsonRound(u8),
    CannotParseJsonOther,
    RaceResultsNotYetAvailable(u8),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::CannotConnectToServer => write!(f, "cannot connect to server"),
            ApiError::CannotParseJsonRound(round) => write!(
                f,
                "results for round {round} could not be parsed and may not exist"
            ),
            ApiError::CannotParseJsonOther => write!(f, "api results could not be parsed"),
            ApiError::RaceResultsNotYetAvailable(round) => {
                write!(f, "results for round {round} are not yet available")
            }
        }
    }
}

impl Error for ApiError {}

#[derive(Debug, Copy, Clone)]
pub enum DeleteError {
    ResultsDeleteWhenResultsDontExist(u8),
    LineupDeleteWhileScoresExist(u8),
    LineupDeleteWhenNextRoundDrafted(u8),
}

impl Display for DeleteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteError::ResultsDeleteWhenResultsDontExist(round) => {
                write!(
                    f,
                    "results for round {} cannot be deleted, as they do not exist",
                    round
                )
            }
            DeleteError::LineupDeleteWhileScoresExist(round) => {
                write!(
                    f,
                    "results for round {} still exist, so the lineup cannot be deleted",
                    round
                )
            }
            DeleteError::LineupDeleteWhenNextRoundDrafted(round) => {
                write!(
                    f,
                    "the lineup for round {} exists, so the lineup for round {} cannot be deleted",
                    round + 1,
                    round
                )
            }
        }
    }
}

impl Error for DeleteError {}
