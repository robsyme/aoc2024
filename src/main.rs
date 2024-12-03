use crate::utils::advent::Solver;

mod utils;
mod location;
mod day1;
mod day2;
mod day3;

fn main() {
    day1::Solver.solve();
    day2::Solver.solve();
    day3::Solver.solve();
}
