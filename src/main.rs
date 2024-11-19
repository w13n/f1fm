use crate::vc::ViewController;
use std::error::Error;

mod api;
mod error;
mod fantasy_season;
mod vc;

fn main() -> Result<(), Box<dyn Error>> {
    iced::run("test", ViewController::update, ViewController::view).expect("TODO: panic message");

    Ok(())
}
