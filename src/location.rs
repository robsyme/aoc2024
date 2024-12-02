use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Location(u32);

impl Location {
    pub fn distance_between((a, b): (&Location, &Location)) -> u32 {
        a.0.abs_diff(b.0)
    }
}

impl Into<u32> for &Location {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone)]
pub struct LocationList {
    pub locations: Vec<Location>,
}

impl LocationList {
    pub fn into_frequencies(self) -> HashMap<Location, u32> {
        self.into_iter().fold(HashMap::new(), |mut acc, loc| {
            *acc.entry(loc).or_insert(0) += 1;
            acc
        })
    }
}

impl Iterator for LocationList {
    type Item = Location;
    fn next(&mut self) -> Option<Self::Item> {
        self.locations.pop()
    }
}

impl FromIterator<Location> for LocationList {
    fn from_iter<T: IntoIterator<Item = Location>>(iter: T) -> Self {
        let mut locations: Vec<Location> = iter.into_iter().collect();
        locations.sort();
        LocationList { locations }
    }
}

pub struct LocationListPair {
    pub left: LocationList,
    pub right: LocationList,
}

impl TryFrom<&str> for LocationListPair {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut left_locations: Vec<Location> = Vec::new();
        let mut right_locations: Vec<Location> = Vec::new();

        for (line_num, line) in value.lines().filter(|line| !line.is_empty()).enumerate() {
            let mut parts = line.split_whitespace();

            let left = parts
                .next()
                .ok_or_else(|| anyhow!("Line {}: Missing first number", line_num + 1))
                .and_then(|n| {
                    n.parse::<u32>()
                        .context(format!("Parse error on line {}: '{}'", line_num + 1, n))
                        .map(Location)
                })?;
            left_locations.push(left);

            let right = parts
                .next()
                .ok_or_else(|| anyhow!("Line {}: Missing second number", line_num + 1))
                .and_then(|n| {
                    n.parse::<u32>()
                        .context(format!("Parse error on line {}: '{}'", line_num + 1, n))
                        .map(Location)
                })?;
            right_locations.push(right);
        }

        Ok(Self {
            left: LocationList::from_iter(left_locations),
            right: LocationList::from_iter(right_locations),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        let input = "1 2\n3 4\n5 6";
        let location_lists = LocationListPair::try_from(input).unwrap();
        assert_eq!(location_lists.left.locations.len(), 3);
        assert_eq!(location_lists.right.locations.len(), 3);
    }

    #[test]
    fn test_try_from_uneven_columns() {
        let input = "1 2\n3 4\n5";
        let result = LocationListPair::try_from(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_invalid_numbers() {
        let input = "1 a\n3 4\n5 6";
        let result = LocationListPair::try_from(input);
        assert!(result.is_err());
    }
}
