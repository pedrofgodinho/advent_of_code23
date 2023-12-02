use super::Solution;

mod day1;
mod day2;

pub const SOLUTIONS: [(Solution, Solution); 2] =
    [(day1::part1, day1::part2), (day2::part1, day2::part2)];
