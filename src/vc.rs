mod builder;
mod landing;
mod season;
mod style;

use crate::fantasy_season::FantasySeason;
use crate::vc::builder::{Builder, BuilderMessage};
use crate::vc::landing::Landing;
use crate::vc::season::{Season, SeasonMessage};
use directories_next::ProjectDirs;
use iced::{Element, Subscription, Task};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

const PADDING: u16 = 7;

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
        seasons_path.push("seasons_v1n");
        let mut seasons_file = Vec::with_capacity(100);

        if std::fs::exists(&seasons_path).unwrap_or_default() {
            let mut file = File::open(seasons_path).unwrap();
            file.read_to_end(&mut seasons_file).unwrap();
        } else {
            seasons_path.pop();
            seasons_path.push("seasons_v1");
            if let Ok(mut file) = File::open(seasons_path) {
                file.read_to_end(&mut seasons_file).unwrap();
            }
        }

        let seasons: Vec<FantasySeason> = postcard::from_bytes(&seasons_file).unwrap_or_default();
        let season_names = seasons.iter().map(|s| String::from(s.get_name())).collect();

        ViewController {
            seasons,
            window: Window::Landing(Landing::new(season_names)),
            save_path,
        }
    }
    pub fn view(&self) -> Element<VCMessage> {
        iced::widget::container(match &self.window {
            Window::Season(season) => season.view(),
            Window::Builder(builder) => builder.view(),
            Window::Landing(landing) => landing.view(),
        })
        .padding(PADDING)
        .into()
    }

    pub fn subscription(&self) -> Subscription<VCMessage> {
        iced::time::every(Duration::from_secs(5)).map(|_| VCMessage::Save)
    }

    pub fn update(&mut self, message: VCMessage) -> Task<VCMessage> {
        match message {
            VCMessage::Season(sm) => {
                if let Window::Season(s) = &mut self.window {
                    s.update(sm).map(VCMessage::Season)
                } else {
                    Task::none()
                }
            }
            VCMessage::Builder(bm) => {
                if let Window::Builder(b) = &mut self.window {
                    b.update(bm);
                }
                Task::none()
            }
            VCMessage::Save => {
                let first_season = if let Window::Season(s) = &self.window {
                    vec![s.get_season()]
                } else {
                    Vec::new()
                };
                let all_seasons: Vec<_> = first_season
                    .into_iter()
                    .chain(self.seasons.iter())
                    .collect();

                if std::fs::create_dir_all(&self.save_path).is_ok() {
                    let mut n_path = self.save_path.clone();
                    let mut path = self.save_path.clone();

                    n_path.push("seasons_v1n");
                    path.push("seasons_v1");

                    File::create(&n_path)
                        .unwrap()
                        .write_all(&postcard::to_stdvec(&all_seasons).unwrap())
                        .unwrap();

                    let _ = std::fs::rename(n_path, path);
                }
                Task::none()
            }
            VCMessage::WindowExit => {
                match &mut self.window {
                    Window::Season(s) => {
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
                    Window::Builder(_b) => {
                        self.window = Window::Landing(Landing::new(
                            self.seasons
                                .iter()
                                .map(|s| String::from(s.get_name()))
                                .collect(),
                        ));
                        Task::none()
                    }
                    Window::Landing(_l) => Task::none(), //  we cant close the landing
                }
            }
            VCMessage::OpenSeason(idx) => {
                self.window = Window::Season(Season::new(self.seasons.remove(idx)));
                Task::batch(vec![
                    Task::done(VCMessage::Season(SeasonMessage::DownloadFirstRace)),
                    Task::done(VCMessage::Season(SeasonMessage::DownloadRaceNames)),
                ])
            }
            VCMessage::DeleteSeason(idx) => {
                self.seasons.remove(idx);
                if let Window::Landing(l) = &mut self.window {
                    l.delete(idx);
                }
                Task::none()
            }
            VCMessage::OpenBuilder => {
                self.window = Window::Builder(Builder::new());
                Task::none()
            }
            VCMessage::CreateFromBuilder => match &mut self.window {
                Window::Builder(b) => {
                    self.window = Window::Season(Season::new(b.create()));
                    Task::batch(vec![
                        Task::done(VCMessage::Season(SeasonMessage::DownloadFirstRace)),
                        Task::done(VCMessage::Season(SeasonMessage::DownloadRaceNames)),
                    ])
                }
                _ => {
                    panic!("IMPOSSIBLE: CFB MESSAGE PASSED WHEN NO BUILDER IS PRESENT")
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
    Season(SeasonMessage),
    Builder(BuilderMessage),
    Save,
    WindowExit,
    OpenSeason(usize),
    DeleteSeason(usize),
    OpenBuilder,
    CreateFromBuilder,
}
