use crate::fantasy_season::race_results::RaceResults;
use std::error::Error;

mod error;
mod fantasy_season;

fn main() -> Result<(), Box<dyn Error>> {
    let x = RaceResults::build(17, 2024);
    if let Err(err) = x {
        println!("Error: cannot get race results. {}", err);
    } else {
        println!("{:#?}", x);
    }

    Ok(())
}
