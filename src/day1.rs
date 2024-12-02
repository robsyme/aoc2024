use crate::location::Location;
use crate::location::LocationListPair;
use crate::utils::advent;
use anyhow::Result;
pub struct Solver;

impl advent::Solver<1> for Solver {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_part_one(&self, input: &str) -> Result<Self::Part1> {
        let location_list_pair = LocationListPair::try_from(input)?;

        let left = location_list_pair.left.locations.iter();
        let right = location_list_pair.right.locations.iter();

        let total_distance = left.zip(right).map(Location::distance_between).sum();
        Ok(total_distance)
    }

    fn solve_part_two(&self, input: &str) -> Result<Self::Part2> {
        let location_list_pair = LocationListPair::try_from(input)?;
        let frequencies = location_list_pair.right.into_frequencies();
        let similarity_score = location_list_pair
            .left
            .locations
            .iter()
            .map(|location| {
                let frequency = frequencies.get(location).unwrap_or(&0);
                let count: u32 = location.into();
                count * frequency
            })
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
