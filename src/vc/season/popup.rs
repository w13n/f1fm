use crate::fantasy_season::draft::Drafter;
use crate::vc::MONO_FONT;
use crate::vc::season::SeasonAction;
use crate::vc::{CONTENT, PADDING};
use crate::vc::{CONTENT_INPUT_PADDED, style};
use iced::{Alignment, Element, Length, widget};
use replace_all_drafter::ReplaceAllDrafter;
use roll_on_drafter::RollOnDrafter;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod replace_all_drafter;
pub mod roll_on_drafter;

pub(super) struct Popup {
    title: String,
    kind: PopupKind,
}

enum PopupKind {
    RollOnDrafter(RollOnDrafter),
    ReplaceAllDrafter(ReplaceAllDrafter),
}

#[derive(Clone, Debug)]
pub enum PopupMessage {
    RollOn(roll_on_drafter::ROMessage),
    ReplaceAll(replace_all_drafter::RAMessage),
    Close,
}

pub enum PopupAction {
    UpdateLineup,
    None,
}

impl Popup {
    pub fn new_roll_on(
        previous_lineup: HashMap<String, Vec<u8>>,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup {
            title: "draft new drivers".to_string(),
            kind: PopupKind::RollOnDrafter(RollOnDrafter::new(previous_lineup, enforce_uniqueness)),
        }
    }

    pub fn new_replace_all(
        team_names: Vec<String>,
        team_size: usize,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup {
            title: "draft new drivers".to_string(),
            kind: PopupKind::ReplaceAllDrafter(ReplaceAllDrafter::new(
                team_names,
                team_size,
                enforce_uniqueness,
            )),
        }
    }

    pub fn replace_all_from(
        team_lineups: HashMap<String, Vec<String>>,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup {
            title: "edit lineup".to_string(),
            kind: PopupKind::ReplaceAllDrafter(ReplaceAllDrafter::from(
                team_lineups,
                enforce_uniqueness,
            )),
        }
    }

    pub fn get_drafter(self) -> Box<dyn Drafter> {
        match self.kind {
            PopupKind::RollOnDrafter(ro) => Box::new(ro.get_drafter()),
            PopupKind::ReplaceAllDrafter(ra) => Box::new(ra.get_drafter()),
        }
    }
    pub fn view(&self) -> Element<PopupMessage> {
        let top = crate::vc::top_row(self.title.clone(), MONO_FONT, PopupMessage::Close);

        let main = widget::container(match &self.kind {
            PopupKind::RollOnDrafter(ro) => ro.view().map(PopupMessage::RollOn),
            PopupKind::ReplaceAllDrafter(ra) => ra.view().map(PopupMessage::ReplaceAll),
        });

        widget::column![top, main].into()
    }

    pub fn update(&mut self, message: PopupMessage) -> SeasonAction {
        match message {
            PopupMessage::RollOn(msg) => match &mut self.kind {
                PopupKind::RollOnDrafter(ro) => {
                    let action = ro.update(msg);
                    self.handle_action(action)
                }
                PopupKind::ReplaceAllDrafter(_) => panic!("RollOn msg passed to ReplaceAll"),
            },
            PopupMessage::ReplaceAll(msg) => match &mut self.kind {
                PopupKind::RollOnDrafter(_) => panic!("ReplaceAll msg passed to ReplaceAll"),
                PopupKind::ReplaceAllDrafter(ra) => {
                    let action = ra.update(msg);
                    self.handle_action(action)
                }
            },
            PopupMessage::Close => SeasonAction::ClosePopup,
        }
    }

    fn handle_action(&mut self, action: PopupAction) -> SeasonAction {
        match action {
            PopupAction::UpdateLineup => SeasonAction::UpdateLineup,
            PopupAction::None => SeasonAction::None,
        }
    }
}

fn lineup_view<T: Debug + Clone + 'static>(
    mut content: Vec<(String, Vec<Element<T>>)>,
    can_draft: bool,
    message: T,
) -> Element<T> {
    let mut team_section = Vec::new();

    let mut length = 0;

    for (team_name, _) in &content {
        length = length.max(team_name.len());
    }

    for (team_name, elements) in &mut content {
        let mut row = Vec::new();

        row.push(
            widget::text!("{:>length$}", team_name)
                .size(CONTENT)
                .align_y(Alignment::Center)
                .height(CONTENT_INPUT_PADDED)
                .into(),
        );

        row.push(widget::horizontal_space().width(PADDING).into());

        row.append(elements);
        team_section.push(widget::Row::from_vec(row).spacing(PADDING).into());
    }

    widget::column![
        widget::vertical_space(),
        widget::Column::from_vec(team_section).spacing(PADDING),
        widget::vertical_space(),
        widget::button("finish")
            .on_press_maybe(can_draft.then_some(message))
            .style(style::button::primary),
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .spacing(PADDING)
    .into()
}
