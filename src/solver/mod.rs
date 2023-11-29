mod solutions;

use self::solutions::SOLUTIONS;
use rayon::prelude::*;
use std::{fs, path::Path};

type Solution = fn(&str) -> String;

pub fn solve(day: usize, in_dir: &Path) -> Result<(String, String), String> {
    if day == 0 || day > SOLUTIONS.len() {
        return Err(format!("Invalid day: {day}").to_owned());
    }

    let input = fs::read_to_string(in_dir.join(format!("{day}.txt"))).map_err(|e| e.to_string())?;

    let solution1 = SOLUTIONS[day - 1].0(&input);
    let solution2 = SOLUTIONS[day - 1].1(&input);

    Ok((solution1, solution2))
}

pub fn solve_all_sequential(in_dir: &Path, parallel: bool) {
    if parallel {
        SOLUTIONS
            .par_iter()
            .enumerate()
            .for_each(|(idx, solutions)| {
                let input = fs::read_to_string(in_dir.join(format!("{}.txt", idx + 1))).unwrap();
                println!("Day {} part 1: {}", idx + 1, solutions.0(&input));
                println!("Day {} part 2: {}", idx + 1, solutions.1(&input));
            });
    } else {
        SOLUTIONS.iter().enumerate().for_each(|(idx, solutions)| {
            let input = fs::read_to_string(in_dir.join(format!("{}.txt", idx + 1))).unwrap();
            println!("Day {} part 1: {}", idx + 1, solutions.0(&input));
            println!("Day {} part 2: {}", idx + 1, solutions.1(&input));
        });
    }
}
