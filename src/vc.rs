pub mod api;
mod builder;
mod landing;
mod season;
mod style;
mod utils;

use crate::fantasy_season::FantasySeason;
use builder::{Builder, BuilderMessage};
use directories_next::ProjectDirs;
use iced::font::Weight;
use iced::{Alignment, Element, Font, Length, Subscription, Task, widget};
use landing::{Landing, LandingMessage};
use season::{Season, SeasonMessage};
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

const PADDING: u16 = 7;
const TITLE: u16 = 24;
const CONTENT: u16 = 20;
const CONTENT_INPUT_PADDED: u16 = (CONTENT as f64 * 1.7) as u16;

const EXIT_BUTTON_SPACING: f32 = 30.0;
const MONO_FONT: Font = {
    let mut font = Font::with_name("IBM Plex Mono Bold");
    font.weight = Weight::Bold;
    font
};
const F1_FONT: Font = Font::with_name("Formula1");

const SYMB_FONT: Font = {
    let mut font = Font::with_name("Material Symbols Rounded");
    font.weight = Weight::Bold;
    font
};

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
            Window::Season(season) => season.view().map(VCMessage::Season),
            Window::Builder(builder) => builder.view().map(VCMessage::Builder),
            Window::Landing(landing) => landing.view().map(VCMessage::Landing),
        })
        .padding(PADDING)
        .into()
    }

    pub fn subscription(&self) -> Subscription<VCMessage> {
        let save = iced::time::every(Duration::from_secs(5)).map(|_| VCMessage::Save);
        let window = match &self.window {
            Window::Season(s) => s.subscription().map(VCMessage::Season),
            Window::Builder(_) => Subscription::none(),
            Window::Landing(_) => Subscription::none(),
        };

        Subscription::batch(vec![save, window])
    }

    pub fn update(&mut self, message: VCMessage) -> Task<VCMessage> {
        match message {
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
            VCMessage::Season(sm) => {
                if let Window::Season(s) = &mut self.window {
                    let action = s.update(sm);
                    self.handle_action(action)
                } else {
                    Task::none()
                }
            }
            VCMessage::Builder(bm) => {
                if let Window::Builder(b) = &mut self.window {
                    let action = b.update(bm);
                    self.handle_action(action)
                } else {
                    Task::none()
                }
            }
            VCMessage::Landing(lm) => {
                if let Window::Landing(l) = &mut self.window {
                    let action = l.update(lm);
                    self.handle_action(action)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn handle_action(&mut self, action: VCAction) -> Task<VCMessage> {
        match action {
            VCAction::WindowExit => {
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
            VCAction::OpenSeason(idx) => {
                self.window = Window::Season(Season::new(self.seasons.remove(idx)));
                Task::batch(vec![
                    Task::done(VCMessage::Season(SeasonMessage::DownloadFirstRace)),
                    Task::done(VCMessage::Season(SeasonMessage::DownloadRaceNames)),
                ])
            }
            VCAction::DeleteSeason(idx) => {
                self.seasons.remove(idx);
                if let Window::Landing(l) = &mut self.window {
                    l.delete(idx);
                }
                Task::none()
            }
            VCAction::OpenBuilder => {
                self.window = Window::Builder(Builder::new());
                Task::none()
            }
            VCAction::CreateFromBuilder => match &mut self.window {
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
            VCAction::None => Task::none(),
            VCAction::Task(task) => task,
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

pub enum VCAction {
    WindowExit,
    OpenSeason(usize),
    DeleteSeason(usize),
    OpenBuilder,
    CreateFromBuilder,
    Task(Task<VCMessage>),
    None,
}

fn top_row<T: Debug + Clone + 'static>(title: String, font: Font, exit: T) -> Element<'static, T> {
    let exit_button = widget::button(
        widget::text!("\u{e9ba}")
            .align_x(Alignment::Center)
            .font(SYMB_FONT),
    )
    .on_press(exit)
    .style(widget::button::text)
    .width(Length::Fixed(EXIT_BUTTON_SPACING));

    let title = widget::text!("{}", title)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .size(TITLE)
        .font(font);

    widget::row![
        exit_button,
        title,
        widget::horizontal_space().width(Length::Fixed(EXIT_BUTTON_SPACING)),
    ]
    .into()
}
