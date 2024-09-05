use crate::fantasy_season::race_results::DriverResult;

pub enum Scorer {
    FormulaOne,
    RacePosition,
    Improvement,
    Domination,
}

impl Scorer {
    pub fn get_fn(&self) -> fn(u8, &DriverResult) -> i16 {
        match self {
            Scorer::FormulaOne => |_, dr| match dr.final_position {
                1 => 25,
                2 => 18,
                3 => 15,
                4 => 12,
                5 => 10,
                6 => 8,
                7 => 6,
                8 => 4,
                9 => 2,
                10 => 1,
                _ => 0,
            },
            Scorer::RacePosition => |grid_size, dr| grid_size as i16 + 1 - dr.final_position as i16,
            Scorer::Improvement => |grid_size, dr| {
                (grid_size as i16 + 1 - dr.final_position as i16)
                    + (dr.grid_position as i16 - dr.final_position as i16)
            },
            Scorer::Domination => |grid_size, dr| {
                (grid_size as i16 + 1 - dr.final_position as i16)
                    + (grid_size as i16 + 1 - dr.qualifying_position as i16)
            },
        }
    }
}
