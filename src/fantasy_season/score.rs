use crate::fantasy_season::race_results::DriverResult;
use std::fmt::Display;

pub trait Scorer {
    fn score(&self, grid_size: u8, dr: &DriverResult) -> i16;
}

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum ScoreChoice {
    #[default]
    FormulaOne,
    RacePosition,
    Improvement,
    Domination,
}

impl Display for ScoreChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ScoreChoice::FormulaOne => String::from("Formula One"),
            ScoreChoice::RacePosition => String::from("Race Position"),
            ScoreChoice::Improvement => String::from("Improvement"),
            ScoreChoice::Domination => String::from("Domination"),
        };
        write!(f, "{}", str)
    }
}

impl Scorer for ScoreChoice {
    fn score(&self, grid_size: u8, dr: &DriverResult) -> i16 {
        match self {
            ScoreChoice::FormulaOne => match dr.final_position {
                1 => 25,
                2 => 18,
                3 => 15,
                4 => 12,
                5 => 10,
                6 => 8,
                7 => 6,
                8 => 4,
                9 => 2,
                10 => 1,
                _ => 0,
            },
            ScoreChoice::RacePosition => grid_size as i16 + 1 - dr.final_position as i16,
            ScoreChoice::Improvement => {
                (grid_size as i16 + 1 - dr.final_position as i16)
                    + (dr.grid_position as i16 - dr.final_position as i16)
            }
            ScoreChoice::Domination => {
                (grid_size as i16 + 1 - dr.final_position as i16)
                    + (grid_size as i16 + 1 - dr.qualifying_position as i16)
            }
        }
    }
}
