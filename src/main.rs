use crate::fantasy_season::drafter::{DraftChoice, Drafter, Skipper};
use crate::fantasy_season::race_results::RaceResults;
use crate::fantasy_season::scorer::{ScoreChoice, Scorer};
use crate::fantasy_season::FantasySeason;
use std::error::Error;

mod api;
mod error;
mod fantasy_season;

fn main() -> Result<(), Box<dyn Error>> {
    let mut season = FantasySeason::new(
        ScoreChoice::FormulaOne,
        DraftChoice::Skip,
        vec!["Red Bull".to_string(), "Mercedes".to_string()],
        vec![vec![33, 11], vec![44, 63]],
        2024,
        20,
        true,
    );

    season.download(1)?;
    season.score(1)?;
    season.draft(2, Box::new(Skipper {}))?;
    season.download(2)?;
    season.score(2)?;

    let map = season.get_points_at(16);
    println!("{:#?}", map);

    let x = RaceResults::build(1, 2024);
    match x {
        Ok(results) => {
            println!("{:#?}", results);
        }
        Err(err) => {
            println!("Error: cannot get race results. {}", err);
        }
    }

    Ok(())
}
