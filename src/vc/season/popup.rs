use crate::fantasy_season::draft::Drafter;
use crate::vc::PADDING;
use crate::vc::style;
use iced::{Alignment, Element, Length, widget};
use replace_all_drafter::ReplaceAllDrafter;
use roll_on_drafter::RollOnDrafter;
use std::collections::HashMap;

pub mod replace_all_drafter;
pub mod roll_on_drafter;

pub(super) enum Popup {
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
        Popup::RollOnDrafter(RollOnDrafter::new(previous_lineup, enforce_uniqueness))
    }

    pub fn new_replace_all(
        team_names: Vec<String>,
        team_size: usize,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup::ReplaceAllDrafter(ReplaceAllDrafter::new(
            team_names,
            team_size,
            enforce_uniqueness,
        ))
    }

    pub fn replace_all_from(
        team_lineups: HashMap<String, Vec<String>>,
        enforce_uniqueness: bool,
    ) -> Popup {
        Popup::ReplaceAllDrafter(ReplaceAllDrafter::from(team_lineups, enforce_uniqueness))
    }

    pub fn get_drafter(self) -> Box<dyn Drafter> {
        match self {
            Popup::RollOnDrafter(ro) => Box::new(ro.get_drafter()),
            Popup::ReplaceAllDrafter(ra) => Box::new(ra.get_drafter()),
        }
    }
    pub fn view(&self) -> Element<PopupMessage> {
        let button = widget::button(widget::text!["exit"].align_x(Alignment::Center))
            .on_press(PopupMessage::Close)
            .style(style::button::secondary)
            .width(Length::Fixed(75.));
        let main = widget::container(match self {
            Popup::RollOnDrafter(ro) => ro.view(),
            Popup::ReplaceAllDrafter(ra) => ra.view(),
        });

        widget::column![button, main].into()
    }

    pub fn update(&mut self, message: PopupMessage) {
        match message {
            PopupMessage::RollOn(msg) => match self {
                Popup::RollOnDrafter(ro) => ro.update(msg),
                Popup::ReplaceAllDrafter(_) => panic!("RollOn msg passed to ReplaceAll"),
            },
            PopupMessage::ReplaceAll(msg) => match self {
                Popup::RollOnDrafter(_) => panic!("ReplaceAll msg passed to ReplaceAll"),
                Popup::ReplaceAllDrafter(ra) => ra.update(msg),
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
                .align_y(Alignment::Center)
                .height(30)
                .into(),
        );

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
    .into()
}
