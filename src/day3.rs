use crate::utils::advent;
use anyhow::Result;

#[derive(Debug)]
pub struct Solver;

impl advent::Solver<3> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Result<Self::Part1> {
        let mut sum = 0;
        let mut current = input;

        while let Some(mul_pos) = current.find("mul(") {
            if let Ok((product, rest)) = parse_mul_and_remainder(&current[mul_pos..]) {
                sum += product;
                current = rest;
            } else {
                current = current.get(mul_pos + 4..).unwrap_or_default();
            }
        }

        Ok(sum)
    }

    fn solve_part_two(&self, input: &str) -> Result<Self::Part2> {
        let mut sum = 0;
        let mut current = input;
        let mut enabled = true;

        while !current.is_empty() {
            if let Some(rest) = current.strip_prefix("don't()") {
                enabled = false;
                current = rest;
            } else if let Some(rest) = current.strip_prefix("do()") {
                enabled = true;
                current = rest;
            } else if enabled && current.starts_with("mul(") {
                if let Ok((product, rest)) = parse_mul_and_remainder(current) {
                    sum += product;
                    current = rest;
                } else {
                    current = current.strip_prefix("mul(").unwrap_or(current);
                }
            } else {
                // Move to the next character
                current = current.get(1..).unwrap_or_default();
            }
        }

        Ok(sum)
    }
}

fn parse_mul_and_remainder(input: &str) -> Result<(usize, &str)> {
    let (nums, rest) = input
        .strip_prefix("mul(")
        .and_then(|s| s.split_once(')'))
        .ok_or_else(|| anyhow::anyhow!("Invalid mul expression"))?;

    let nums: Vec<usize> = nums
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    match nums[..] {
        [x, y] => Ok((x * y, rest)),
        _ => Err(anyhow::anyhow!("Expected exactly 2 numbers")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::advent::Solver as AdventSolver;

    const TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    const FULL_INPUT: &str = include_str!("../input/day03.txt");

    #[test]
    fn test_part_one() {
        let solver = Solver;
        let result = solver.solve_part_one(TEST_INPUT_1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 161);
    }

    #[test]
    fn test_part_one_full() {
        let solver = Solver;
        let result = solver.solve_part_one(FULL_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 183788984);
    }

    #[test]
    fn test_part_two() {
        let solver = Solver;
        let result = solver.solve_part_two(TEST_INPUT_2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 48);
    }

    #[test]
    fn test_part_two_full() {
        let solver = Solver;
        let result = solver.solve_part_two(FULL_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 62098619);
    }

    #[test]
    fn should_parse_unclosed_mul() {
        assert!(parse_mul_and_remainder("mul(32,64]then").is_err());
    }
}
