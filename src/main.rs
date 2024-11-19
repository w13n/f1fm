use crate::fantasy_season::draft::{DraftChoice, Skipper};
use crate::fantasy_season::race_results::RaceResults;
use crate::fantasy_season::score::ScoreChoice;
use crate::fantasy_season::FantasySeason;
use std::error::Error;
use crate::api::Api;

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

    let map = season.get_points_at(2);
    println!("{:#?}", map);

    println!("{:#?}", Api::new().get_race_names(2024));

    Ok(())
}
