mod builder;
mod landing;
mod season;
mod style;

use crate::fantasy_season::FantasySeason;
use crate::vc::builder::{Builder, BuilderMessage};
use crate::vc::landing::{Landing, LandingMessage};
use crate::vc::season::{Season, SeasonMessage};
use directories_next::ProjectDirs;
use iced::{Element, Subscription, Task};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

pub(super) struct ViewController {
    window: Window,
    seasons: Vec<FantasySeason>,
    save_path: PathBuf,
}

impl Default for ViewController {
    fn default() -> Self {
        ViewController::new()
    }
}

impl ViewController {
    pub(crate) fn new() -> ViewController {
        let save_path = PathBuf::from(ProjectDirs::from("com", "w13n", "F1FM").unwrap().data_dir());

        let mut seasons_path = save_path.clone();
        seasons_path.push("seasons_v1");
        let mut seasons_file = Vec::with_capacity(100);
        File::open(seasons_path)
            .unwrap()
            .read_to_end(&mut seasons_file)
            .unwrap();
        let seasons: Vec<FantasySeason> = postcard::from_bytes(&seasons_file).unwrap();
        let season_names = seasons.iter().map(|s| String::from(s.get_name())).collect();

        ViewController {
            seasons,
            window: Window::Landing(Landing::new(season_names)),
            save_path,
        }
    }
    pub fn view(&self) -> Element<VCMessage> {
        match &self.window {
            Window::Season(season) => season.view().map(VCMessage::Season),
            Window::Builder(builder) => builder.view().map(VCMessage::Builder),
            Window::Landing(landing) => landing.view().map(VCMessage::Landing),
        }
    }

    pub fn subscription(&self) -> Subscription<VCMessage> {
        iced::time::every(Duration::from_secs(1)).map(|_| VCMessage::Save)
    }

    pub fn update(&mut self, message: VCMessage) -> Task<VCMessage> {
        match message {
            VCMessage::Season(sm) => match &mut self.window {
                Window::Season(s) => match sm {
                    SeasonMessage::Exit => {
                        let mut names: Vec<_> = self
                            .seasons
                            .iter()
                            .map(|s| String::from(s.get_name()))
                            .collect();

                        names.insert(0, s.get_season().get_name().to_string());
                        if let Window::Season(s) = std::mem::replace(
                            &mut self.window,
                            Window::Landing(Landing::new(names)),
                        ) {
                            self.seasons.insert(0, s.take_season());
                        }
                        Task::none()
                    }
                    _ => s.update(sm).map(VCMessage::Season),
                },
                _ => {
                    // season may have been closed since this task happened closed, so we do nothing
                    Task::none()
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
                    BuilderMessage::Exit => {
                        self.window = Window::Landing(Landing::new(
                            self.seasons
                                .iter()
                                .map(|s| String::from(s.get_name()))
                                .collect(),
                        ));
                        Task::none()
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
            VCMessage::Landing(lm) => match lm {
                LandingMessage::Pick(idx) => {
                    self.window = Window::Season(Season::new(self.seasons.remove(idx)));
                    Task::batch(vec![
                        Task::done(VCMessage::Season(SeasonMessage::DownloadFirstRace)),
                        Task::done(VCMessage::Season(SeasonMessage::DownloadRaceNames)),
                    ])
                }
                LandingMessage::Build => {
                    self.window = Window::Builder(Builder::new());
                    Task::none()
                }
                LandingMessage::Delete(usize) => {
                    self.seasons.remove(usize);
                    match &mut self.window {
                        Window::Landing(l) => {
                            l.update(lm);
                        }
                        _ => panic!("LanderMessage created for non lander window"),
                    }
                    Task::none()
                }
            },
            VCMessage::Save => {
                let first_season = match &self.window {
                    Window::Season(season) => {
                        vec![season.get_season()]
                    }
                    _ => Vec::new(),
                };
                let all_seasons: Vec<_> = first_season
                    .into_iter()
                    .chain(self.seasons.iter())
                    .collect();

                if std::fs::create_dir_all(&self.save_path).is_ok() {
                    let mut path = self.save_path.clone();
                    path.push("seasons_v1");
                    File::create(&path)
                        .unwrap()
                        .write_all(&postcard::to_stdvec(&all_seasons).unwrap())
                        .unwrap();
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
    Save,
    Season(SeasonMessage),
    Builder(BuilderMessage),
    Landing(LandingMessage),
}
