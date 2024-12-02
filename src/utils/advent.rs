use anyhow::Result;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::path::Path;

pub trait Solver<const DAY: u32> {
    type Part1: Display;
    type Part2: Display;

    fn solve_part_one(&self, input: &str) -> Result<Self::Part1>;
    fn solve_part_two(&self, input: &str) -> Result<Self::Part2>;

    fn solve(&self) {
        let day = DAY;
        println!("Advent of code 2024 day {day}");

        let input_path = format!("input/day{:02}.txt", day);
        let input_path = Path::new(&input_path);
        let input_dir = input_path.parent().expect("Failed to get input directory");
        if !input_dir.exists() {
            println!("Input folder does not exist, creating it...");
        }
        fs::create_dir_all(input_dir).expect("Failed to create input folder");

        if !input_path.exists() {
            println!("Input file for day {day} does not exist, creating it...");
            File::create(&input_path).expect("Failed to create input file for day {day}");
        }

        let input = fs::read_to_string(input_path).expect("Failed to read input file");

        print!("Part 1: ");
        match self.solve_part_one(&input) {
            Ok(part_one_solution) => println!("{}", part_one_solution),
            Err(e) => println!("Failed to solve part 1: {}", e),
        }
        
        print!("Part 2: ");
        match self.solve_part_two(&input) {
            Ok(part_two_solution) => println!("{}", part_two_solution),
            Err(e) => println!("Failed to solve part 2: {}", e),
        }
    }
}
