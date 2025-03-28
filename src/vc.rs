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
        let seasons = Vec::new();
        let seasons_clone = Vec::new();
        ViewController {
            seasons,
            window: Window::Landing(Landing::new(seasons_clone)),
        }
    }
    pub fn view(&self) -> Element<VCMessage> {
        match &self.window {
            Window::Season(season) => season.view().map(VCMessage::Season),
            Window::Builder(builder) => builder.view().map(VCMessage::Builder),
            Window::Landing(landing) => landing.view().map(VCMessage::Landing),
        }
    }

    pub fn update(&mut self, message: VCMessage) -> Task<VCMessage> {
        match message {
            VCMessage::Season(sm) => match &mut self.window {
                Window::Season(s) => s.update(sm).map(VCMessage::Season),
                _ => {
                    panic!("SeasonMessage created for non season")
                }
            },
            VCMessage::Builder(bm) => match &mut self.window {
                Window::Builder(b) => match bm {
                    BuilderMessage::Create => {
                        self.window = Window::Season(Season::new(b.create()));
                        Task::batch(vec![
                            Task::done(VCMessage::Season(SeasonMessage::DownloadFirstRace)),
                            Task::done(VCMessage::Season(SeasonMessage::DownloadRaceNames)),
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
            VCMessage::Landing(lm) => {
                match lm {
                    LandingMessage::Pick(idx) => {
                        self.window = Window::Season(Season::new(self.seasons.remove(idx)))
                    }
                    LandingMessage::Build => self.window = Window::Builder(Builder::new()),
                }
                Task::none()
            }
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
    Season(SeasonMessage),
    Builder(BuilderMessage),
    Landing(LandingMessage),
}
