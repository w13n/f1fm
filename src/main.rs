use std::error::Error;
use crate::fantasy_season::race_results::RaceResults;

mod fantasy_season;
mod error;

fn main() -> Result<(), Box<dyn Error>> {
    let x = RaceResults::build(200, 2024)?;

    println!("{:#?}", x);

    Ok(())
}