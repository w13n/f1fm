use std::collections::HashSet;
use std::hash::Hash;

pub fn is_unique_lineups<T: Iterator>(mut lineups: T) -> bool
where
    <T as Iterator>::Item: Eq + Hash,
{
    let mut seen = HashSet::new();
    lineups.all(|y| seen.insert(y))
}

pub fn is_valid_driver_input(new: &str) -> bool {
    new.is_empty() || is_parsable_driver(new)
}

pub fn is_parsable_driver(new: &str) -> bool {
    new.parse::<u8>().is_ok_and(|num| num < 100)
}
