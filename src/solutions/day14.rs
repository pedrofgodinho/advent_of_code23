use ahash::AHasher;
use ahash::HashMap;
use ahash::HashMapExt;
use ndarray::{Array, Array2, Axis};
use std::hash::Hasher;

use super::Solution;

pub struct Day14 {
    rocks: Array2<Option<bool>>,
    seen: HashMap<u64, usize>,
    seen_vec: Vec<Array2<Option<bool>>>,
}

impl Solution for Day14 {
    fn part1(&mut self) -> String {
        let rocks = &mut self.rocks;
        shift_up(rocks);
        total_load(rocks).to_string()
    }

    fn part2(&mut self) -> String {
        let mut rocks = self.rocks.clone();
        let mut loop_end = 0;

        let loop_start = loop {
            rocks = spin_cycle(rocks);
            let hash = hash(&rocks);
            if let Some(idx) = self.seen.get(&hash) {
                break *idx;
            }
            self.seen.insert(hash, loop_end);
            self.seen_vec.push(rocks.clone());
            loop_end += 1;
        };

        let loop_length = loop_end - loop_start;

        let target = 1000000000;
        let target = (target - loop_start - 1) % loop_length;

        total_load(&self.seen_vec[loop_start + target]).to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        let cols = input.find('\n').unwrap();
        let rows = input.lines().count();
        let rocks = Array::from_iter(input.lines().flat_map(|line| {
            line.bytes().map(|byte| match byte {
                b'O' => Some(true),
                b'#' => Some(false),
                _ => None,
            })
        }))
        .into_shape((rows, cols))
        .unwrap();
        Box::new(Self {
            rocks,
            seen: HashMap::new(),
            seen_vec: Vec::new(),
        })
    }
}

fn hash(rocks: &Array2<Option<bool>>) -> u64 {
    let mut hasher = AHasher::default();
    for (pos, rock) in rocks.iter().enumerate() {
        if let Some(true) = rock {
            hasher.write_usize(pos);
        }
    }
    hasher.finish()
}

fn total_load(rocks: &Array2<Option<bool>>) -> usize {
    let mut sum = 0;
    let len = rocks.shape()[0];
    for col in rocks.columns() {
        for (row_idx, row) in col.iter().enumerate() {
            if let Some(true) = row {
                sum += len - row_idx;
            }
        }
    }
    sum
}

fn shift_up(rocks: &mut Array2<Option<bool>>) {
    for mut col in rocks.columns_mut() {
        let mut next_rock_position = 0;
        for row_idx in 0..col.len() {
            match col[row_idx] {
                Some(true) => {
                    col[row_idx] = None;
                    col[next_rock_position] = Some(true);
                    next_rock_position += 1;
                }
                Some(false) => {
                    next_rock_position = row_idx + 1;
                }
                None => {}
            }
        }
    }
}

fn spin_cycle(mut rocks: Array2<Option<bool>>) -> Array2<Option<bool>> {
    for _ in 0..4 {
        shift_up(&mut rocks);
        rocks = rocks.reversed_axes();
        rocks.invert_axis(Axis(1));
    }
    rocks
}
