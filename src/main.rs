use crate::fantasy_season::race_results::RaceResults;
use std::error::Error;

mod api;
mod error;
mod fantasy_season;

fn main() -> Result<(), Box<dyn Error>> {
    let x = RaceResults::build(21, 2024);
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
