use crate::vc::season::SeasonMessage;
use iced::{Element, Task};
use std::collections::HashMap;

pub struct Landing {
    names: Vec<String>,
}

impl Landing {
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    pub fn update(&mut self, message: LandingMessage) -> Task<LandingMessage> {
        todo!()
    }
    pub fn view(&self) -> Element<LandingMessage> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum LandingMessage {
    Pick(usize),
    Build,
}
