mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub type ParserFn = fn(String) -> Box<dyn Solution>;

pub trait Solution: Sync + Send {
    fn parse(input: String) -> Box<dyn Solution>
    where
        Self: Sized;
    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;
}

pub fn get_parser_fns() -> [ParserFn; 15] {
    [
        day1::Day1::parse,
        day2::Day2::parse,
        day3::Day3::parse,
        day4::Day4::parse,
        day5::Day5::parse,
        day6::Day6::parse,
        day7::Day7::parse,
        day8::Day8::parse,
        day9::Day9::parse,
        day10::Day10::parse,
        day11::Day11::parse,
        day12::Day12::parse,
        day13::Day13::parse,
        day14::Day14::parse,
        day15::Day15::parse,
    ]
}
