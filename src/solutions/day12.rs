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
        count_solutions_aux(&self, 0, 0, 0)
    }
}

fn hash(row: &StatusRow, idx1: usize, idx2: usize, count: usize) -> u64 {
    let mut hasher = AHasher::default();
    row.hash(&mut hasher);
    idx1.hash(&mut hasher);
    idx2.hash(&mut hasher);
    count.hash(&mut hasher);
    hasher.finish()
}

#[cached(
    type = "SizedCache<u64, usize>",
    create = "{SizedCache::with_size(10000)}",
    convert = r#"{ hash(s, springs_idx, groups_idx, damaged_count) }"#
)]
fn count_solutions_aux(
    s: &StatusRow,
    springs_idx: usize,
    groups_idx: usize,
    damaged_count: usize,
) -> usize {
    if springs_idx == s.springs.len() {
        if groups_idx == s.groups.len() {
            return 1;
        } else {
            return 0;
        }
    }

    match s.springs[springs_idx] {
        SpringStatus::Functional => {
            let count = s.springs[springs_idx..]
                .iter()
                .position(|&s| s != SpringStatus::Functional)
                .unwrap_or(1);
            if groups_idx < s.groups.len() && damaged_count == s.groups[groups_idx] {
                // There's still groups to go, and current group checks out
                count_solutions_aux(&s, springs_idx + count, groups_idx + 1, 0)
            } else if damaged_count == 0 {
                // Either there's no more groups to go or current group doesn't match next group
                // But count is zero, it's ok
                count_solutions_aux(&s, springs_idx + count, groups_idx, 0)
            } else {
                // Error
                0
            }
        }
        SpringStatus::Damaged => {
            // ###. -> .
            // damaged_count + 3
            // idx + 3
            let count = s.springs[springs_idx..]
                .iter()
                .position(|&s| s != SpringStatus::Damaged)
                .unwrap();
            count_solutions_aux(&s, springs_idx + count, groups_idx, damaged_count + count)
        }
        SpringStatus::Unknown => {
            // Pretend ? is a #
            let broken_count =
                count_solutions_aux(&s, springs_idx + 1, groups_idx, damaged_count + 1);

            // Pretend ? is a .
            let functional_count =
                if groups_idx < s.groups.len() && damaged_count == s.groups[groups_idx] {
                    // We finished a group
                    count_solutions_aux(&s, springs_idx + 1, groups_idx + 1, 0)
                } else if damaged_count == 0 {
                    // We were not in a group
                    count_solutions_aux(&s, springs_idx + 1, groups_idx, 0)
                } else {
                    // We finished a group but with the wrong value, therefore ? cannot be a .
                    0
                };
            broken_count + functional_count
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
