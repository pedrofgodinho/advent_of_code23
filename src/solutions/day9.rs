use super::Solution;

pub struct Day9 {}

impl Solution for Day9 {
    fn part1(&self, input: &str) -> String {
        let reports = input.lines().map(Report::from_line).collect::<Vec<_>>();
        reports
            .iter()
            .map(|report| report.find_next_number())
            .sum::<isize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let reports = input.lines().map(Report::from_line).collect::<Vec<_>>();
        reports
            .iter()
            .map(|report| report.find_previous_number())
            .sum::<isize>()
            .to_string()
    }

    fn parse(&mut self) {}
}

impl Day9 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Report {
    sequence: Vec<isize>,
}

impl Report {
    fn from_line(line: &str) -> Self {
        Self {
            sequence: line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    fn get_sub_report(&self) -> Self {
        Self {
            sequence: self
                .sequence
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>(),
        }
    }

    fn find_next_number(&self) -> isize {
        if self.is_zeroes() {
            return 0;
        }
        let report = self.get_sub_report();
        let next = report.find_next_number();
        self.sequence[self.sequence.len() - 1] + next
    }

    fn find_previous_number(&self) -> isize {
        if self.is_zeroes() {
            return 0;
        }
        let report = self.get_sub_report();
        let next = report.find_previous_number();
        self.sequence[0] - next
    }

    fn is_zeroes(&self) -> bool {
        self.sequence.iter().all(|&n| n == 0)
    }
}
