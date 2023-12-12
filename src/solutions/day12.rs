use ahash::AHasher;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use cached::proc_macro::cached;
use cached::SizedCache;

use super::Solution;

pub struct Day12 {}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        let rows = input.lines().map(StatusRow::from_line).collect::<Vec<_>>();
        rows.into_iter()
            .map(|row| row.count_solutions())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rows = input
            .lines()
            .map(StatusRow::from_line_repeating)
            .collect::<Vec<_>>();
        rows.into_iter()
            .enumerate()
            .map(|(_, row)| row.count_solutions())
            .sum::<usize>()
            .to_string()
    }

    fn setup(&mut self) {}
}

impl Day12 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum SpringStatus {
    Functional,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct StatusRow {
    springs: Vec<SpringStatus>,
    groups: Vec<usize>,
}

impl SpringStatus {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'.' => Some(Self::Functional),
            b'#' => Some(Self::Damaged),
            b'?' => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl StatusRow {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        Self {
            springs: parts
                .next()
                .unwrap()
                .bytes()
                .filter_map(SpringStatus::from_byte)
                .chain([SpringStatus::Functional])
                .collect(),
            groups: parts
                .next()
                .unwrap()
                .split(',')
                .filter_map(|n| n.parse().ok())
                .collect(),
        }
    }

    fn from_line_repeating(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let springs = parts.next().unwrap();
        let groups = parts.next().unwrap();
        Self {
            springs: springs
                .bytes()
                .chain([b'?'])
                .chain(springs.bytes())
                .chain([b'?'])
                .chain(springs.bytes())
                .chain([b'?'])
                .chain(springs.bytes())
                .chain([b'?'])
                .chain(springs.bytes())
                .filter_map(SpringStatus::from_byte)
                .chain([SpringStatus::Functional])
                .collect(),
            groups: groups
                .split(',')
                .chain(groups.split(','))
                .chain(groups.split(','))
                .chain(groups.split(','))
                .chain(groups.split(','))
                .filter_map(|n| n.parse().ok())
                .collect(),
        }
    }

    fn count_solutions(self) -> usize {
        count_solutions_aux(&self.springs, &self.groups, 0)
    }
}

fn hash(row: &[SpringStatus], groups: &[usize], count: usize) -> u64 {
    let mut hasher = AHasher::default();
    row.hash(&mut hasher);
    groups.hash(&mut hasher);
    count.hash(&mut hasher);
    hasher.finish()
}

#[cached(
    type = "SizedCache<u64, usize>",
    create = "{SizedCache::with_size(10000)}",
    convert = r#"{ hash(row, groups, count) }"#
)]
fn count_solutions_aux(row: &[SpringStatus], groups: &[usize], count: usize) -> usize {
    if row.is_empty() {
        if groups.is_empty() || (groups.len() == 1 && groups[0] == count) {
            return 1;
        }
        return 0;
    }
    if groups.is_empty() {
        if row.contains(&SpringStatus::Damaged) {
            return 0;
        }
        return 1;
    }

    match row[0] {
        SpringStatus::Damaged => {
            if count > groups[0] {
                return 0;
            }
            let skip = row
                .iter()
                .position(|&s| s != SpringStatus::Damaged)
                .unwrap_or(1);
            return count_solutions_aux(&row[skip..], groups, count + skip);
        }
        SpringStatus::Functional => {
            let skip = row
                .iter()
                .position(|&s| s != SpringStatus::Functional)
                .unwrap_or(1);
            if count == 0 {
                return count_solutions_aux(&row[skip..], groups, 0);
            } else if count == groups[0] {
                return count_solutions_aux(&row[skip..], &groups[1..], 0);
            } else {
                return 0;
            }
        }
        SpringStatus::Unknown => {
            if count != 0 {
                if groups[0] == count {
                    return count_solutions_aux(&row[1..], &groups[1..], 0);
                } else {
                    return count_solutions_aux(&row[1..], groups, count + 1);
                }
            } else {
                if groups[0] == count {
                    return count_solutions_aux(&row[1..], groups, 0);
                } else {
                    return count_solutions_aux(&row[1..], groups, 0)
                        + count_solutions_aux(&row[1..], groups, 1);
                }
            }
        }
    }
}

impl Display for SpringStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpringStatus::Functional => ".",
                SpringStatus::Damaged => "#",
                SpringStatus::Unknown => "?",
            }
        )
    }
}
