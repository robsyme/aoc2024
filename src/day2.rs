use crate::utils::advent;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
pub struct Solver;

/// Represents a single step between two height levels, indicating whether it goes up, down, or stays level
#[derive(Debug, PartialEq)]
enum Step {
    Up(i32),
    Down(i32),
    Level,
}

impl From<(i32, i32)> for Step {
    fn from((prev, next): (i32, i32)) -> Self {
        use std::cmp::Ordering::*;
        match next.cmp(&prev) {
            Greater => Self::Up(next - prev),
            Less => Self::Down(prev - next),
            Equal => Self::Level,
        }
    }
}

/// Tracks the current direction state of a sequence of steps
#[derive(Debug, Clone, Copy)]
enum State {
    Ascending,
    Descending,
    Unknown,
}

/// Represents a report of height levels that can be analyzed for safety
#[derive(Debug, PartialEq)]
struct Report(Vec<i32>);

impl Report {
    /// Converts a sequence of height levels into steps, excluding levels at specified indices
    fn steps(&self, skip_indices: &[usize]) -> Vec<Step> {
        self.0
            .iter()
            .enumerate()
            .filter(|(i, _)| !skip_indices.contains(i))
            .map(|(_, &level)| level)
            .tuple_windows::<(i32, i32)>()
            .map(Step::from)
            .collect()
    }

    /// Generates all possible combinations of steps when skipping up to max_dampening levels
    fn dampened_steps(&self, max_dampening: usize) -> impl Iterator<Item = Vec<Step>> + '_ {
        (0..=max_dampening).flat_map(move |skip_count| {
            (0..self.0.len())
                .combinations(skip_count)
                .map(move |skip_list| self.steps(&skip_list))
        })
    }

    /// Determines if a report is "safe" based on two criteria:
    /// 1. Steps can't exceed max_distance in either direction
    /// 2. Direction can only change once (from unknown to ascending/descending)
    /// 3. Can skip up to max_dampening levels
    fn is_safe(&self, max_dampening: usize, max_distance: i32) -> bool {
        if self.0.is_empty() {
            return true;
        }

        self.dampened_steps(max_dampening).any(|directions| {
            use State::*;
            let mut state = Unknown;
            directions.iter().all(|direction| match (direction, state) {
                (Step::Level, _) => false,
                (Step::Up(d), _) if *d > max_distance => false,
                (Step::Down(d), _) if *d > max_distance => false,
                (Step::Up(_), Descending) => false,
                (Step::Down(_), Ascending) => false,
                (Step::Up(_), _) => {
                    state = Ascending;
                    true
                }
                (Step::Down(_), _) => {
                    state = Descending;
                    true
                }
            })
        })
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let levels = s
            .split_whitespace()
            .map(|s| s.parse().context("Failed to parse number"))
            .collect::<Result<_>>()?;
        Ok(Self(levels))
    }
}

impl advent::Solver<2> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Result<Self::Part1> {
        let count = input
            .lines()
            .filter_map(|line| Report::from_str(line).ok())
            .filter(|report| report.is_safe(0, 3))
            .count();
        Ok(count)
    }

    fn solve_part_two(&self, input: &str) -> Result<Self::Part2> {
        let count = input
            .lines()
            .filter_map(|line| Report::from_str(line).ok())
            .filter(|report| report.is_safe(1, 3))
            .count();
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::advent::Solver as AdventSolver;

    const TEST_INPUT: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9";

    #[test]
    fn test_part_one() {
        let solver = Solver;
        let result = solver.solve_part_one(TEST_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let solver = Solver;
        let result = solver.solve_part_two(TEST_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn safe_reports() {
        assert!(Report::from_str("1 2 3 4 5").unwrap().is_safe(0, 3));
        assert!(Report::from_str("5 4 2").unwrap().is_safe(0, 3));
    }

    #[test]
    fn reports_that_level_out_are_unsafe() {
        assert!(!Report::from_str("1 2 3 4 4").unwrap().is_safe(0, 3));
        assert!(!Report::from_str("5 4 2 2").unwrap().is_safe(0, 3));
    }

    #[test]
    fn reports_that_move_up_and_down_are_unsafe() {
        assert!(!Report::from_str("1 2 3 4 1").unwrap().is_safe(0, 3));
        assert!(!Report::from_str("10 9 1 7").unwrap().is_safe(0, 3));
    }

    #[test]
    fn empty_report_is_safe() {
        assert!(Report::from_str("").unwrap().is_safe(0, 3));
    }
}
