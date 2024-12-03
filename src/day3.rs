use crate::utils::advent;
use anyhow::Result;

#[derive(Debug)]
pub struct Solver;

impl advent::Solver<3> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Result<Self::Part1> {
        let mut count = 0;
        let sum = input
            .match_indices("mul(")
            .filter_map(|(i, _)| {
                if let Ok(product) = parse_mul(&input[i..]) {
                    count += 1;
                    Some(product)
                } else {
                    None
                }
            })
            .sum();

        Ok(sum)
    }

    fn solve_part_two(&self, input: &str) -> Result<Self::Part2> {
        let sum = input
            .char_indices()
            .scan(true, |enabled, (i, _)| {
                let s = &input[i..];
                if s.starts_with("don't()") {
                    *enabled = false;
                    Some(None)
                } else if s.starts_with("do()") {
                    *enabled = true;
                    Some(None)
                } else if s.starts_with("mul(") && *enabled {
                    Some(parse_mul(s).ok())
                } else {
                    Some(None)
                }
            })
            .flatten()
            .sum();

        Ok(sum)
    }
}

fn parse_mul(input: &str) -> Result<usize> {
    let (nums, _) = input
        .strip_prefix("mul(")
        .and_then(|s| s.split_once(')'))
        .ok_or_else(|| anyhow::anyhow!("Invalid mul expression"))?;

    let nums: Vec<usize> = nums
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    match nums[..] {
        [x, y] => Ok(x * y),
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
    fn test_basic_parsing() {
        assert!(matches!(parse_mul("mul(2,4)"), Ok(8)));
        assert!(matches!(parse_mul("mul(3,7)"), Ok(21)));
    }

    #[test]
    fn should_parse_unclosed_mul() {
        assert!(parse_mul("mul(32,64]then").is_err());
    }
}
