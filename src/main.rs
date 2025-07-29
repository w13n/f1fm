#![warn(missing_docs)]

use crate::vc::ViewController;
use iced::font::Weight;
use iced::theme::Palette;
use iced::{Font, Theme};

mod api;
pub mod fantasy_season;
mod vc;

const F1_FONT: &[u8] = include_bytes!("../assets/Formula1-Regular.ttf");
const MONOSPACE_FONT: &[u8] = include_bytes!("../assets/IBMPlexMono-Bold.ttf");

const SYMBOLS_FONT: &[u8] = include_bytes!("../assets/MaterialSymbolsRounded-Bold.ttf");

fn main() {
    iced::application("test", ViewController::update, ViewController::view)
        .theme(|_| {
            Theme::custom(
                String::from("Classic West"),
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
        .font(F1_FONT)
        .font(MONOSPACE_FONT)
        .font(SYMBOLS_FONT)
        .default_font({
            let mut font = Font::with_name("IBM Plex Mono Bold");
            font.weight = Weight::Bold;
            font
        })
        .run()
        .expect("TODO: panic message");
}
