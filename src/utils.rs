use std::collections::HashSet;

pub fn is_unique_lineups(lineups: Vec<Vec<u8>>) -> bool {
    let mut seen = HashSet::new();
    lineups.iter().all(|x| x.iter().all(|y| seen.insert(y)))
}

pub fn is_valid_driver_str(new: &str) -> bool {
    new.is_empty() || new.parse::<u8>().is_ok_and(|num| num < 100)
}
