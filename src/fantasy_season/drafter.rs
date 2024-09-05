use crate::error::DraftError;

pub enum Drafter {
    Skip,
}

impl Drafter {
    pub fn get_fn(&self) -> fn(&str, &Vec<u8>) -> Result<Vec<u8>, DraftError> {
        match self {
            Drafter::Skip => |_, prev_lineup| Ok(prev_lineup.clone()),
        }
    }
}
