use ndarray::{Array, Array2};

use super::Solution;

pub struct Day16 {
    cells: Array2<CellData>,
}

impl Solution for Day16 {
    fn part1(&mut self) -> String {
        self.navigate(Direction::Right, (0, 0)).to_string()
    }

    fn part2(&mut self) -> String {
        let nrows = self.cells.nrows();
        let ncols = self.cells.ncols();
        (0..nrows)
            .flat_map(|row| {
                [
                    (Direction::Right, (row, 0)),
                    (Direction::Left, (row, ncols - 1)),
                ]
            })
            .chain((0..ncols).flat_map(|col| {
                [
                    (Direction::Down, (0, col)),
                    (Direction::Up, (nrows - 1, col)),
                ]
            }))
            .map(|(dir, pos)| self.navigate(dir, pos))
            .max()
            .unwrap()
            .to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        let cols = input.find('\n').unwrap();
        let rows = input.lines().count();
        let cells = Array::from_iter(input.lines().flat_map(|line| {
            line.bytes().map(|byte| {
                CellData::new(match byte {
                    b'.' => CellType::Empty,
                    b'|' => CellType::VSplit,
                    b'-' => CellType::HSplit,
                    b'/' => CellType::MirrorUp,
                    b'\\' => CellType::MirrorDown,
                    _ => panic!(),
                })
            })
        }))
        .into_shape((rows, cols))
        .unwrap();
        Box::new(Self { cells })
    }
}

impl Day16 {
    fn navigate(&mut self, direction: Direction, idx: (usize, usize)) -> usize {
        let mut stack = vec![(direction, idx)];

        while let Some((mut direction, idx)) = stack.pop() {
            let cell = &mut self.cells[idx];
            if cell.visited_dir & (1 << direction as usize) != 0 {
                continue;
            }
            cell.visited_dir |= 1 << direction as usize;
            match cell.cell_type {
                CellType::Empty => (),
                CellType::VSplit => match direction {
                    Direction::Left | Direction::Right => {
                        if let Some(idx) = self.try_add_direction_to_idx(&Direction::Up, idx) {
                            stack.push((Direction::Up, idx));
                        }
                        direction = Direction::Down;
                    }
                    Direction::Up | Direction::Down => (),
                },
                CellType::HSplit => match direction {
                    Direction::Up | Direction::Down => {
                        if let Some(idx) = self.try_add_direction_to_idx(&Direction::Right, idx) {
                            stack.push((Direction::Right, idx));
                        }
                        direction = Direction::Left;
                    }
                    Direction::Left | Direction::Right => (),
                },
                CellType::MirrorUp => match direction {
                    Direction::Up => direction = Direction::Right,
                    Direction::Down => direction = Direction::Left,
                    Direction::Left => direction = Direction::Down,
                    Direction::Right => direction = Direction::Up,
                },
                CellType::MirrorDown => match direction {
                    Direction::Up => direction = Direction::Left,
                    Direction::Down => direction = Direction::Right,
                    Direction::Left => direction = Direction::Up,
                    Direction::Right => direction = Direction::Down,
                },
            }
            if let Some(idx) = self.try_add_direction_to_idx(&direction, idx) {
                stack.push((direction, idx));
            }
        }

        self.cells
            .iter_mut()
            .filter_map(|cell| {
                let visited = cell.visited_dir != 0;
                cell.visited_dir = 0;
                if visited {
                    Some(1)
                } else {
                    None
                }
            })
            .count()
    }

    fn try_add_direction_to_idx(
        &self,
        direction: &Direction,
        idx: (usize, usize),
    ) -> Option<(usize, usize)> {
        let offset = direction.as_offset();
        match (
            idx.0.checked_add_signed(offset.0),
            idx.1.checked_add_signed(offset.1),
        ) {
            (Some(row), Some(col)) => {
                if row >= self.cells.nrows() || col >= self.cells.ncols() {
                    None
                } else {
                    Some((row, col))
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CellType {
    Empty,
    VSplit,
    HSplit,
    MirrorUp,   // /
    MirrorDown, // \
}

#[derive(Debug)]
struct CellData {
    cell_type: CellType,
    visited_dir: u8,
}

impl CellData {
    fn new(cell_type: CellType) -> Self {
        Self {
            cell_type,
            visited_dir: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}
