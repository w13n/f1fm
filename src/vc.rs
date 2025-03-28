mod builder;
mod landing;
mod season;
mod style;

use crate::fantasy_season::FantasySeason;
use crate::vc::builder::{Builder, BuilderMessage};
use crate::vc::landing::{Landing, LandingMessage};
use crate::vc::season::{Season, SeasonMessage};
use iced::{Element, Task};

pub(super) struct ViewController {
    window: Window,
    seasons: Vec<FantasySeason>,
}

impl Default for ViewController {
    fn default() -> Self {
        ViewController::new()
    }
}

impl ViewController {
    pub(crate) fn new() -> ViewController {
        ViewController {
            seasons: Vec::new(),
            window: Window::Builder(Builder::new()),
        }
    }
    pub fn view(&self) -> Element<VCMessage> {
        match &self.window {
            Window::Season(season) => season.view().map(VCMessage::SeasonMessage),
            Window::Builder(builder) => builder.view().map(VCMessage::BuilderMessage),
        }
    }

    pub fn update(&mut self, message: VCMessage) -> Task<VCMessage> {
        match message {
            VCMessage::SeasonMessage(sm) => match &mut self.window {
                Window::Season(s) => s.update(sm).map(VCMessage::SeasonMessage),
                _ => {
                    panic!("SeasonMessage created for non season")
                }
            },
            VCMessage::BuilderMessage(bm) => match &mut self.window {
                Window::Builder(b) => match bm {
                    BuilderMessage::Create => {
                        self.window = Window::Season(Season::new(b.create()));
                        Task::batch(vec![
                            Task::done(VCMessage::SeasonMessage(SeasonMessage::DownloadFirstRace)),
                            Task::done(VCMessage::SeasonMessage(SeasonMessage::DownloadRaceNames)),
                        ])
                    }
                    _ => {
                        b.update(bm);
                        Task::none()
                    }
                },
                _ => {
                    panic!("BuilderMessage created for non builder")
                }
            },
        }
    }
}

enum Window {
    Season(Season),
    Builder(Builder),
    Landing(Landing),
}

#[derive(Debug, Clone)]
pub enum VCMessage {
    SeasonMessage(SeasonMessage),
    BuilderMessage(BuilderMessage),
    LandingMessage(LandingMessage),
}
