mod season;

use iced::Element;
use crate::fantasy_season::draft::DraftChoice;
use crate::fantasy_season::FantasySeason;
use crate::fantasy_season::score::ScoreChoice;
use crate::vc::season::{Season, SeasonMessage};

pub(crate) struct ViewController {
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
        let mut season = FantasySeason::new(
            "F1FL".to_string(),
            ScoreChoice::FormulaOne,
            DraftChoice::Skip,
            vec!["Red Bull".to_string(), "Mercedes".to_string()],
            vec![vec![33, 11], vec![44, 63]],
            2024,
            20,
            true,
        );

        ViewController {
            seasons: Vec::new(),
            window: Window::Season(Season::new(season))
        }
    }
    pub fn view(&self) -> Element<VCMessage> {
        match &self.window {
            Window::Season(Season) => {
                Season.view().map(VCMessage::SeasonMessage)
            }
        }
    }

    pub fn update(&mut self, message: VCMessage) {
        match message {
            VCMessage::SeasonMessage(sm) => {
                match &mut self.window {
                    Window::Season(s) => {s.update(sm)}
                }
            }
        }
    }
}

enum Window {
    Season(Season),
}

#[derive(Debug, Copy, Clone)]
pub enum VCMessage {
    SeasonMessage(SeasonMessage)
}