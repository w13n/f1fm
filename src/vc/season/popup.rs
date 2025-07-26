use crate::fantasy_season::draft::Drafter;
use crate::vc::{CONTENT, PADDING};
use crate::vc::{CONTENT_INPUT_PADDED, style};
use crate::vc::{F1_FONT, MONO_FONT, TITLE};
use iced::{Alignment, Element, Length, widget};
use replace_all_drafter::ReplaceAllDrafter;
use roll_on_drafter::RollOnDrafter;
use std::collections::HashMap;

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
    UpdateLineup,
    Close,
}

impl Popup {
    pub fn new_roll_on(
        previous_lineup: HashMap<String, Vec<u8>>,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup {
            title: "Draft New Drivers".to_string(),
            kind: PopupKind::RollOnDrafter(RollOnDrafter::new(previous_lineup, enforce_uniqueness)),
        }
    }

    pub fn new_replace_all(
        team_names: Vec<String>,
        team_size: usize,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup {
            title: "Draft New Drivers".to_string(),
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
            title: "Edit Lineup".to_string(),
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
            PopupKind::RollOnDrafter(ro) => ro.view(),
            PopupKind::ReplaceAllDrafter(ra) => ra.view(),
        });

        widget::column![top, main].into()
    }

    pub fn update(&mut self, message: PopupMessage) {
        match message {
            PopupMessage::RollOn(msg) => match &mut self.kind {
                PopupKind::RollOnDrafter(ro) => ro.update(msg),
                PopupKind::ReplaceAllDrafter(_) => panic!("RollOn msg passed to ReplaceAll"),
            },
            PopupMessage::ReplaceAll(msg) => match &mut self.kind {
                PopupKind::RollOnDrafter(_) => panic!("ReplaceAll msg passed to ReplaceAll"),
                PopupKind::ReplaceAllDrafter(ra) => ra.update(msg),
            },
            PopupMessage::Close | PopupMessage::UpdateLineup => {
                panic!("draft msg passed to draft window")
            }
        }
    }
}

fn lineup_view(
    mut content: Vec<(String, Vec<Element<PopupMessage>>)>,
    can_draft: bool,
) -> Element<PopupMessage> {
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
            .on_press_maybe(can_draft.then_some(PopupMessage::UpdateLineup))
            .style(style::button::primary),
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .spacing(PADDING)
    .into()
}
