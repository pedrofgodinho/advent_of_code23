use std::fmt::Display;

use ndarray::{Array, Array2};

use super::Solution;

pub struct Day10 {}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let mut map = PipeMap::from_str(input);
        ((map.walk_loop_and_fix() + 1) / 2).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut map = PipeMap::from_str(input);
        map.walk_loop_and_fix();
        map.enclosed().to_string()
    }

    fn setup(&mut self) {}
}

impl Day10 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum MapCell {
    Pipe(Pipe, bool),
    Starting,
    Empty,
}

#[derive(Debug)]
struct PipeMap {
    map: Array2<MapCell>,
    start_pos: (usize, usize),
}

impl Pipe {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'|' => Some(Self::Vertical),
            b'-' => Some(Self::Horizontal),
            b'L' => Some(Self::NorthEast),
            b'J' => Some(Self::NorthWest),
            b'7' => Some(Self::SouthWest),
            b'F' => Some(Self::SouthEast),
            _ => None,
        }
    }

    fn directions(&self) -> [(isize, isize); 2] {
        match self {
            Self::Vertical => [(1, 0), (-1, 0)],
            Self::Horizontal => [(0, 1), (0, -1)],
            Self::NorthWest => [(-1, 0), (0, -1)],
            Self::NorthEast => [(-1, 0), (0, 1)],
            Self::SouthWest => [(1, 0), (0, -1)],
            Self::SouthEast => [(1, 0), (0, 1)],
        }
    }
}

impl MapCell {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'S' => Some(Self::Starting),
            b'.' => Some(Self::Empty),
            _ => Pipe::from_byte(byte).map(|pipe| Self::Pipe(pipe, false)),
        }
    }
}

impl PipeMap {
    fn from_str(str: &str) -> Self {
        let cols = str.find('\n').unwrap();
        let rows = str.lines().count();
        let map = Array::from_iter(str.bytes().filter_map(MapCell::from_byte))
            .into_shape((rows, cols))
            .unwrap();

        let start = map
            .iter()
            .position(|&cell| cell == MapCell::Starting)
            .unwrap();
        let start_pos = (start / cols, start % cols);
        Self { map, start_pos }
    }

    fn try_add_position(&self, a: (usize, usize), b: (isize, isize)) -> Option<(usize, usize)> {
        let rows = self.map.shape()[0];
        let cols = self.map.shape()[1];
        let res = (a.0 as isize + b.0, a.1 as isize + b.1);
        if res.0 >= 0 && (res.0 as usize) < rows && res.1 >= 0 && (res.1 as usize) < cols {
            Some((res.0 as usize, res.1 as usize))
        } else {
            None
        }
    }

    fn walk_loop_and_fix(&mut self) -> usize {
        let start_pos = self.start_pos;
        let mut last_dir = (1, 0);
        let mut current_pos = start_pos;
        let mut counter = 0;
        // Pick starting direction
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(pos) = self.try_add_position(start_pos, dir) {
                if let MapCell::Pipe(pipe, _) = self.map[pos] {
                    if pipe
                        .directions()
                        .iter()
                        .any(|dir2| dir == (-dir2.0, -dir2.1))
                    {
                        current_pos = pos;
                        last_dir = dir;
                        break;
                    }
                }
            }
        }
        let first_dir = last_dir;
        loop {
            counter += 1;
            if let MapCell::Pipe(pipe, _) = self.map[current_pos] {
                self.map[current_pos] = MapCell::Pipe(pipe, true);
                last_dir = pipe
                    .directions()
                    .into_iter()
                    .find(|&dir| dir != (-last_dir.0, -last_dir.1))
                    .unwrap();
            }
            current_pos = self.try_add_position(current_pos, last_dir).unwrap();
            if current_pos == start_pos {
                break;
            }
        }
        let starting_pipe = match (first_dir, last_dir) {
            ((1, 0), (1, 0)) | ((-1, 0), (-1, 0)) => Pipe::Vertical,
            ((0, 1), (0, 1)) | ((0, -1), (0, -1)) => Pipe::Horizontal,
            ((-1, 0), (0, 1)) | ((0, -1), (1, 0)) => Pipe::NorthWest,
            ((-1, 0), (0, -1)) | ((0, 1), (1, 0)) => Pipe::NorthEast,
            ((1, 0), (0, 1)) | ((0, -1), (-1, 0)) => Pipe::SouthWest,
            ((1, 0), (0, -1)) | ((0, 1), (-1, 0)) => Pipe::SouthEast,
            _ => unreachable!(),
        };
        self.map[self.start_pos] = MapCell::Pipe(starting_pipe, true);
        counter
    }

    fn enclosed(&self) -> usize {
        let mut counter = 0;
        for row in self.map.rows() {
            let mut enclosed = false;
            for cell in row {
                match cell {
                    MapCell::Pipe(pipe, true) => match pipe {
                        Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast => {
                            enclosed = !enclosed;
                            // print!("{cell}")
                        }
                        _ => (), //print!("{cell}"),
                    },
                    MapCell::Starting => print!("S"),
                    _ => {
                        if enclosed {
                            counter += 1;
                            // print!("I");
                        } else {
                            // print!(".");
                        }
                    }
                }
            }
            // println!();
        }
        counter
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vertical => "┃",
                Self::Horizontal => "━",
                Self::NorthEast => "┗",
                Self::NorthWest => "┛",
                Self::SouthEast => "┏",
                Self::SouthWest => "┓",
            }
        )
    }
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Starting => write!(f, "S"),
            Self::Pipe(pipe, _) => write!(f, "{}", pipe),
        }
    }
}

impl Display for PipeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
