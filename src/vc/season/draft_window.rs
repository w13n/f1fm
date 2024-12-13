
mod roll_on;
mod replace_all;

pub enum DraftWindow {
    RollOn(roll_on::RollOn),
    ReplaceAll(replace_all::ReplaceAll)
}

enum DWMessage {
    RollOnMessage()
}

impl DraftWindow {

}