use ndarray::{s, Array, Array2};

use super::Solution;

pub struct Day13 {}

impl Solution for Day13 {
    fn part1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|str| Rocks::from_str(str).reflection_point())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|str| Rocks::from_str(str).smudge())
            .sum::<usize>()
            .to_string()
    }

    fn parse(&mut self) {}
}

impl Day13 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Rocks {
    map: Array2<bool>,
}

impl Rocks {
    fn from_str(str: &str) -> Self {
        let cols = str.find('\n').unwrap();
        let rows = str.lines().count();
        let map = Array::from_iter(
            str.lines()
                .flat_map(|line| line.bytes().map(|byte| byte == b'#')),
        )
        .into_shape((rows, cols))
        .unwrap();
        Self { map }
    }

    fn reflection_point(&self) -> usize {
        let shape = self.map.shape();
        // horizontal reflection
        for split_idx in 1..shape[0] {
            let diff = std::cmp::min(split_idx, shape[0] - split_idx);
            let left = self.map.slice(s![split_idx - diff..split_idx;-1, ..]);
            let right = self.map.slice(s![split_idx..split_idx + diff, ..]);

            if left == right {
                return 100 * split_idx;
            }
        }

        // vertical reflection
        for split_idx in 1..shape[1] {
            let diff = std::cmp::min(split_idx, shape[1] - split_idx);
            let left = self.map.slice(s![.., split_idx - diff..split_idx;-1]);
            let right = self.map.slice(s![.., split_idx..split_idx + diff]);

            if left == right {
                return split_idx;
            }
        }

        0
    }

    fn smudge(&self) -> usize {
        let shape = self.map.shape();
        // horizontal reflection
        for split_idx in 1..shape[0] {
            let diff = std::cmp::min(split_idx, shape[0] - split_idx);
            let left = self.map.slice(s![split_idx - diff..split_idx;-1, ..]);
            let right = self.map.slice(s![split_idx..split_idx + diff, ..]);

            let shape = left.shape();
            let mut differences = 0;
            for row in 0..shape[0] {
                for col in 0..shape[1] {
                    if left[[row, col]] != right[[row, col]] {
                        differences += 1;
                    }
                    if differences > 1 {
                        break;
                    }
                }
            }
            if differences == 1 {
                return 100 * split_idx;
            }
        }

        // vertical reflection
        for split_idx in 1..shape[1] {
            let diff = std::cmp::min(split_idx, shape[1] - split_idx);
            let left = self.map.slice(s![.., split_idx - diff..split_idx;-1]);
            let right = self.map.slice(s![.., split_idx..split_idx + diff]);

            let shape = left.shape();
            let mut differences = 0;
            for row in 0..shape[0] {
                for col in 0..shape[1] {
                    if left[[row, col]] != right[[row, col]] {
                        differences += 1;
                    }
                    if differences > 1 {
                        break;
                    }
                }
            }
            if differences == 1 {
                return split_idx;
            }
        }

        0
    }
}
