use crate::utils::advent::Solver;

mod utils;
mod location;
mod day1;
mod day2;

fn main() {
    day1::Solver.solve();
    day2::Solver.solve();
}
