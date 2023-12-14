use ndarray::{Array, Array2};

use super::Solution;

pub struct Day11 {
    universe_1: Universe,
    universe_2: Universe,
}

impl Solution for Day11 {
    fn part1(&mut self) -> String {
        self.universe_1.expand(1);
        let galaxies = &self.universe_1.galaxies;
        let len = galaxies.len();
        let mut sum = 0;
        for i in 0..len {
            for j in (i + 1)..len {
                let a = galaxies[i];
                let b = galaxies[j];
                let distance =
                    (b.0 as isize - a.0 as isize).abs() + (b.1 as isize - a.1 as isize).abs();
                sum += distance;
            }
        }
        sum.to_string()
    }

    fn part2(&mut self) -> String {
        self.universe_2.expand(1_000_000 - 1);
        let galaxies = &self.universe_2.galaxies;
        let len = galaxies.len();
        let mut sum = 0;
        for i in 0..len {
            for j in (i + 1)..len {
                let a = galaxies[i];
                let b = galaxies[j];
                let distance =
                    (b.0 as isize - a.0 as isize).abs() + (b.1 as isize - a.1 as isize).abs();
                sum += distance;
            }
        }
        sum.to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        let universe = Universe::from_str(&input);
        Box::new(Self {
            universe_1: universe.clone(),
            universe_2: universe,
        })
    }
}

#[derive(Debug, Clone)]
struct Universe {
    unexpanded_cells: Array2<bool>,
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn from_str(str: &str) -> Self {
        let cols = str.find('\n').unwrap();
        let rows = str.lines().count();
        let unexpanded_cells = Array::from_iter(
            str.lines()
                .flat_map(|line| line.bytes().map(|byte| byte == b'#')),
        )
        .into_shape((rows, cols))
        .unwrap();
        let mut galaxies = Vec::new();
        for (row_idx, row) in unexpanded_cells.rows().into_iter().enumerate() {
            for (col_idx, col) in row.into_iter().enumerate() {
                if *col {
                    galaxies.push((row_idx, col_idx));
                }
            }
        }
        Self {
            unexpanded_cells,
            galaxies,
        }
    }

    fn expand(&mut self, to_add: usize) {
        let mut galaxies = self.galaxies.clone();
        for (idx, _) in self
            .unexpanded_cells
            .rows()
            .into_iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|cell| !cell))
        {
            for (galaxy_idx, galaxy) in self.galaxies.iter().enumerate() {
                if galaxy.0 > idx {
                    galaxies[galaxy_idx].0 += to_add;
                }
            }
        }

        for (idx, _) in self
            .unexpanded_cells
            .columns()
            .into_iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|cell| !cell))
        {
            for (galaxy_idx, galaxy) in self.galaxies.iter().enumerate() {
                if galaxy.1 > idx {
                    galaxies[galaxy_idx].1 += to_add;
                }
            }
        }
        self.galaxies = galaxies;
    }
}
