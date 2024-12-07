use crate::vc::ViewController;
use std::error::Error;
use iced::Theme;
use iced::theme::Palette;

mod api;
mod error;
mod fantasy_season;
mod vc;

fn main() -> Result<(), Box<dyn Error>> {
    iced::application("test", ViewController::update, ViewController::view)
        .theme(|_| {
            Theme::custom(
                String::from("Eunomia"),
                Palette {
                    background: iced::color!(0x423E3B),
                    text: iced::color!(0xFCF7F8),
                    primary: iced::color!(0xFF6319),
                    success: iced::color!(0x64A6BD),
                    danger: iced::color!(0xB4869F),
                },
            )
        })
        .run()
        .expect("TODO: panic message");

    Ok(())
}
