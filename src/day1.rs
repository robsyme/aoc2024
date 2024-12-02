use crate::utils::advent;
use anyhow::Result;
use std::collections::HashMap;

pub struct Solver;

impl advent::Solver<1> for Solver {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_part_one(&self, input: &str) -> Result<Self::Part1> {
        let mut first_column: Vec<Result<u32, String>> = Vec::new();
        let mut second_column: Vec<Result<u32, String>> = Vec::new();

        for (line_num, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
            let mut parts = line.split_whitespace();

            let first_num = parts
                .next()
                .ok_or_else(|| format!("Line {}: Missing first number", line_num + 1))
                .and_then(|n| {
                    n.parse::<u32>()
                        .map_err(|e| format!("Line {}: {}", line_num + 1, e))
                });
            first_column.push(first_num);

            let second_num = parts
                .next()
                .ok_or_else(|| format!("Line {}: Missing second number", line_num + 1))
                .and_then(|n| {
                    n.parse::<u32>()
                        .map_err(|e| format!("Line {}: {}", line_num + 1, e))
                });
            second_column.push(second_num);
        }

        let mut first_nums: Vec<u32> = first_column.into_iter().filter_map(Result::ok).collect();
        let mut second_nums: Vec<u32> = second_column.into_iter().filter_map(Result::ok).collect();

        first_nums.sort();
        second_nums.sort();

        let distances = first_nums
            .into_iter()
            .zip(second_nums.into_iter())
            .map(|(a, b)| a.abs_diff(b))
            .sum();

        Ok(distances)
    }

    fn solve_part_two(&self, input: &str) -> Result<Self::Part2> {
        let mut first_column: Vec<Result<u32, String>> = Vec::new();
        let mut second_column_counts: HashMap<u32, u32> = HashMap::new();

        for (line_num, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
            let mut parts = line.split_whitespace();
            let first_num = parts
                .next()
                .ok_or_else(|| format!("Line {}: Missing first number", line_num + 1))
                .and_then(|n| {
                    n.parse::<u32>()
                        .map_err(|e| format!("Line {}: {}", line_num + 1, e))
                });
            first_column.push(first_num);

            let second_num = parts
                .next()
                .ok_or_else(|| format!("Line {}: Missing second number", line_num + 1))
                .and_then(|n| {
                    n.parse::<u32>()
                        .map_err(|e| format!("Line {}: {}", line_num + 1, e))
                });
            if let Ok(num) = second_num {
                *second_column_counts.entry(num).or_insert(0) += 1;
            }
        }

        let similarity_score = first_column
            .into_iter()
            .filter_map(|num| num.ok())
            .map(|num| num * second_column_counts.get(&num).unwrap_or(&0))
            .sum();

        Ok(similarity_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::advent::Solver as AdventSolver;

    #[test]
    fn test_part_one() {
        let solver = Solver;
        let result =
            AdventSolver::solve_part_one(&solver, "3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        let solver = Solver;
        let result =
            AdventSolver::solve_part_two(&solver, "3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 31);
    }
}
