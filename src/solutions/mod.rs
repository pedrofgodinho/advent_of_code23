mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub trait Solution: Sync + Send {
    fn setup(&mut self);
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 11] {
    [
        Box::new(day1::Day1),
        Box::new(day2::Day2 {}),
        Box::new(day3::Day3::new()),
        Box::new(day4::Day4::new()),
        Box::new(day5::Day5::new()),
        Box::new(day6::Day6::new()),
        Box::new(day7::Day7::new()),
        Box::new(day8::Day8::new()),
        Box::new(day9::Day9::new()),
        Box::new(day10::Day10::new()),
        Box::new(day11::Day11::new()),
    ]
}
