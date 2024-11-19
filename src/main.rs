use crate::fantasy_season::draft::{DraftChoice, Skipper};
use crate::fantasy_season::score::ScoreChoice;
use crate::fantasy_season::FantasySeason;
use std::error::Error;
use crate::api::Api;
use crate::vc::ViewController;

mod api;
mod error;
mod fantasy_season;
mod vc;

fn main() -> Result<(), Box<dyn Error>> {

    let vc = ViewController::new();
    iced::run("test", ViewController::update, ViewController::view).expect("TODO: panic message");

    Ok(())
}
