use super::Solution;

pub struct Day6 {}

impl Solution for Day6 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<isize>().unwrap());
        let distances = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<isize>().unwrap());

        let races = times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect::<Vec<_>>();

        races
            .iter()
            .map(|race| race.ways_to_beat())
            .product::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let time = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse::<isize>()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse::<isize>()
            .unwrap();
        let race = Race { time, distance };
        race.ways_to_beat().to_string()
    }

    fn setup(&mut self) {}
}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Race {
    time: isize,
    distance: isize,
}

impl Race {
    fn ways_to_beat(&self) -> usize {
        let (first, second) = quadratic(-1, self.time, -self.distance);
        (second - 1.0).ceil() as usize - first as usize
    }
}

fn quadratic(a: isize, b: isize, c: isize) -> (f64, f64) {
    let delta = b * b - 4 * a * c;
    let first = (-b as f64 + (delta as f64).sqrt()) / (2 * a) as f64;
    let second = (-b as f64 - (delta as f64).sqrt()) / (2 * a) as f64;
    (first, second)
}
