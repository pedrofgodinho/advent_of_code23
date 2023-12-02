mod day1;
mod day2;
mod day3;

pub trait Solution: Sync + Send {
    fn setup(&mut self);
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 3] {
    [
        Box::new(day1::Day1),
        Box::new(day2::Day2 {}),
        Box::new(day3::Day3::new()),
    ]
}
