mod day1;
mod day2;

pub trait Solution: Sync + Send {
    fn setup(self: &mut Self);
    fn part1(self: &Self, input: &str) -> String;
    fn part2(self: &Self, input: &str) -> String;
}

pub fn get_solutions() -> [Box<dyn Solution>; 2] {
    [Box::new(day1::Day1), Box::new(day2::Day2 {})]
}
