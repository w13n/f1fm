use crate::vc::ViewController;
use iced::Theme;
use iced::theme::Palette;
use std::error::Error;

mod api;
mod error;
mod fantasy_season;
mod utils;
mod vc;

#[allow(clippy::unreadable_literal)]
fn main() {
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
        .subscription(ViewController::subscription)
        .run()
        .expect("TODO: panic message");
}
