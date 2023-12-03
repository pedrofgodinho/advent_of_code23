use std::collections::HashSet;

use super::Solution;
use itertools::*;
use ndarray::{Array, Array2};

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        let cols = input.find('\n').unwrap();
        let rows = input.lines().count();
        let bytes = Array::from_iter(input.bytes().filter(|&b| b != b'\n'))
            .into_shape((cols, rows))
            .unwrap();
        let mut around_symbols = Array2::from_elem(bytes.raw_dim(), false);

        bytes.indexed_iter().for_each(|((i, j), &b)| match b {
            b'0'..=b'9' | b'.' => (),
            _ => {
                for offset in (-1..=1).cartesian_product(-1..=1).filter(|&c| c != (0, 0)) {
                    let i = (i as isize + offset.0) as usize;
                    let j = (j as isize + offset.1) as usize;
                    if (0..cols).contains(&i) && (0..rows).contains(&j) {
                        around_symbols[[i, j]] = true;
                    }
                }
            }
        });

        let mut total = 0;
        for (bytes, arounds) in bytes.rows().into_iter().zip(around_symbols.rows()) {
            let mut number = 0;
            let mut is_around = false;

            for (byte, around) in bytes.into_iter().zip(arounds) {
                is_around |= around;
                if let Some(digit) = (*byte as char).to_digit(10) {
                    number *= 10;
                    number += digit;
                } else {
                    if is_around {
                        total += number;
                    }
                    number = 0;
                    is_around = false;
                }
            }
        }

        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let cols = input.find('\n').unwrap();
        let rows = input.lines().count();
        let bytes = Array::from_iter(input.bytes().filter(|&b| b != b'\n'))
            .into_shape((cols, rows))
            .unwrap();
        let mut gear_numbers = Array2::from_elem(bytes.raw_dim(), HashSet::new());
        let mut gear_count = 0;

        bytes.indexed_iter().for_each(|((i, j), &b)| {
            if b == b'*' {
                for offset in (-1..=1).cartesian_product(-1..=1).filter(|&c| c != (0, 0)) {
                    let i = (i as isize + offset.0) as usize;
                    let j = (j as isize + offset.1) as usize;
                    if (0..cols).contains(&i) && (0..rows).contains(&j) {
                        gear_numbers[[i, j]].insert(gear_count);
                    }
                }
                gear_count += 1;
            }
        });

        let mut gear_groups = Vec::new();
        gear_groups.resize(gear_count, Vec::new());

        for (bytes, gears) in bytes.rows().into_iter().zip(gear_numbers.rows()) {
            let mut number = 0;
            let mut number_gears: Option<HashSet<_>> = None;

            for (byte, gears) in bytes.into_iter().zip(gears) {
                if let Some(digit) = (*byte as char).to_digit(10) {
                    if let Some(number_gears) = number_gears.as_mut() {
                        number_gears.extend(gears);
                    } else {
                        number_gears = Some(gears.clone());
                    }
                    number *= 10;
                    number += digit;
                } else {
                    if let Some(number_gears) = number_gears {
                        for gear in number_gears {
                            gear_groups[gear].push(number);
                        }
                    }
                    number = 0;
                    number_gears = None;
                }
            }
        }

        gear_groups
            .into_iter()
            .filter(|group| group.len() == 2)
            .map(|group| group[0] * group[1])
            .sum::<u32>()
            .to_string()
    }

    fn setup(&mut self) {}
}
