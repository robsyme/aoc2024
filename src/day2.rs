use crate::utils::advent;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::str::FromStr;
pub struct Solver;

#[derive(Debug, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn windows(&self, skip_list: Vec<usize>) -> Vec<(i32, i32)> {
        self.levels
            .iter()
            .enumerate()
            .filter(|(i, _)| !skip_list.contains(&i))
            .map(|(_, &level)| level)
            .tuple_windows()
            .collect()
    }

    pub fn variant_windows(&self, max_dampner_level: usize) -> Vec<Vec<(i32, i32)>> {
        (0..=max_dampner_level)
            .flat_map(|skip_count| {
                (0..self.len())
                    .into_iter()
                    .combinations(skip_count)
                    .map(|skip_list: Vec<usize>| self.windows(skip_list))
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.levels.len()
    }

    pub fn is_safe(&self, max_dampner_level: usize) -> bool {
        self.variant_windows(max_dampner_level)
            .into_iter()
            .any(|windows| {
                StatusIterator {
                    windows,
                    increasing_count: 0,
                    decreasing_count: 0,
                    max_diff: 3,
                }
                .all(|status| status != LevelStatus::Unsafe)
            })
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|s| s.parse::<i32>().context("Failed to parse number"))
            .collect::<Result<_, _>>()?;
        Ok(Report { levels: numbers })
    }
}

struct StatusIterator {
    windows: Vec<(i32, i32)>,
    increasing_count: usize,
    decreasing_count: usize,
    max_diff: i32,
}

impl<'a> Iterator for StatusIterator {
    type Item = LevelStatus;

    fn next(&mut self) -> Option<Self::Item> {
        let (previous_value, current_value) = self.windows.pop()?;
        let diff = current_value - previous_value;
        if diff.abs() > self.max_diff {
            return Some(LevelStatus::Unsafe);
        }
        match diff {
            0 => Some(LevelStatus::Unsafe),
            _ if diff > 0 && self.decreasing_count > 0 => Some(LevelStatus::Unsafe),
            _ if diff < 0 && self.increasing_count > 0 => Some(LevelStatus::Unsafe),
            _ if diff > 0 => {
                self.increasing_count += 1;
                Some(LevelStatus::Increasing)
            }
            _ if diff < 0 => {
                self.decreasing_count += 1;
                Some(LevelStatus::Decreasing)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum LevelStatus {
    Increasing,
    Decreasing,
    Unsafe,
}

impl advent::Solver<2> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    // 326
    fn solve_part_one(&self, input: &str) -> Result<Self::Part1> {
        let count = input
            .lines()
            .map(|line| Report::from_str(line))
            .filter(|res| match res {
                Ok(report) => report.is_safe(0),
                Err(_) => false,
            })
            .count();
        Ok(count)
    }

    // 381
    fn solve_part_two(&self, input: &str) -> Result<Self::Part2> {
        let count = input
            .lines()
            .map(|line| Report::from_str(line))
            .filter(|res| match res {
                Ok(report) => report.is_safe(1),
                Err(_) => false,
            })
            .count();
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::advent::Solver as AdventSolver;

    #[test]
    fn test_part_one() {
        let solver = Solver;
        let result = AdventSolver::solve_part_one(
            &solver,
            "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let solver = Solver;
        let result = AdventSolver::solve_part_two(
            &solver,
            "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }
    
    #[test]
    fn safe_reports() {
        let report = Report::from_str("1 2 3 4 5").unwrap();
        assert_eq!(report.is_safe(0), true);
        let report = Report::from_str("5 4 2").unwrap();
        assert_eq!(report.is_safe(0), true);
    }

    #[test]
    fn reports_that_level_out_are_unsafe() {
        let report = Report::from_str("1 2 3 4 4").unwrap();
        assert_eq!(report.is_safe(0), false);
        let report = Report::from_str("5 4 2 2").unwrap();
        assert_eq!(report.is_safe(0), false);
    }

    #[test]
    fn reports_that_move_up_and_down_are_unsafe() {
        let report = Report::from_str("1 2 3 4 1").unwrap();
        assert_eq!(report.is_safe(0), false);
        let report = Report::from_str("10 9 1 7").unwrap();
        assert_eq!(report.is_safe(0), false);
    }

    #[test]
    fn empty_report_is_safe() {
        let report = Report::from_str("").unwrap();
        assert_eq!(report.is_safe(0), true);
    }

    #[test]
    fn feature_test() {
        let report = Report::from_str("1 2 3 4 5").unwrap();
        let windows = report.variant_windows(2);
    }
}