use clap::Parser;
use rayon::{prelude::*, ThreadPoolBuilder};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use crate::solutions::get_solutions;

mod solutions;
#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to solve. Do not specify to solve all challenges.
    #[arg(short, long)]
    day: Option<usize>,
    /// The part to solve. Do not specify to solve both parts. Only applies when solving a single challenge.
    #[arg(short, long, value_parser=day_parser)]
    part: Option<u8>,
    /// Directory with the input files in format "{day}.txt".
    #[arg(short, long, default_value = "./in")]
    in_dir: PathBuf,
    /// If threaded flag is active, challenges will be solved in parallel. Only applies when solving all challenges.
    #[arg(short, long, action)]
    threaded: bool,
}

fn day_parser(s: &str) -> Result<u8, String> {
    match s {
        "1" => Ok(1),
        "2" => Ok(2),
        _ => Err("Part must be 1 or 2".to_owned()),
    }
}

fn get_input_for_day(dir: &Path, day: usize) -> Result<String, ()> {
    fs::read_to_string(dir.join(format!("{day}.txt"))).map_err(|_err| ())
}

macro_rules! timed {
    ($to_print:expr, $to_time:block) => {
        {
            let start = Instant::now();
            $to_time
            let duration = Instant::now().duration_since(start);
            println!("{}: {:.04}ms", $to_print, duration.as_secs_f64() * 1000.0);
        }
    };
}

fn main() {
    let args = Args::parse();

    match args.day {
        Some(day) => solve_day(day, args.part, &args.in_dir),
        None => solve_all(args.threaded, &args.in_dir),
    }
}

fn solve_day(day: usize, part: Option<u8>, in_dir: &Path) {
    let mut solutions = get_solutions();

    if day == 0 || day > solutions.len() {
        println!("Invalid day: {day}");
        return;
    }

    let solution = &mut solutions[day - 1];

    timed!("Setup ran in", {
        solution.setup();
    });

    let input;
    timed!("Read input in", {
        input = match get_input_for_day(in_dir, day) {
            Ok(input) => input,
            Err(_) => {
                println!("Could not find input for day {}", day);
                return;
            }
        };
    });

    timed!("Solved in", {
        match part {
            Some(1) => println!("Day {} part 1: {}", day, solution.part1(&input)),
            Some(2) => println!("Day {} part 2: {}", day, solution.part2(&input)),
            None => println!(
                "Day {} part 1: {}\nDay {} part 2: {}",
                day,
                solution.part1(&input),
                day,
                solution.part2(&input)
            ),
            _ => panic!(),
        }
    });
}

fn solve_all(threaded: bool, in_dir: &Path) {
    let mut solutions = get_solutions();
    let _ = ThreadPoolBuilder::new().build_global();

    timed!("Setup ran in", {
        solutions.iter_mut().for_each(|solution| solution.setup());
    });

    let inputs;
    timed!("Read inputs in", {
        inputs = (1..=solutions.len())
            .map(|day| get_input_for_day(in_dir, day))
            .collect::<Vec<_>>();
    });

    timed!("Solved all days in", {
        if threaded {
            solutions.par_iter_mut().enumerate().zip(inputs).for_each(
                |((day, solution), input)| match input {
                    Ok(input) => {
                        println!(
                            "Day {} part 1: {}\nDay {} part 2: {}",
                            day + 1,
                            solution.part1(&input),
                            day + 1,
                            solution.part2(&input)
                        );
                    }
                    Err(_) => println!("Input not found for day {}", day + 1),
                },
            );
        } else {
            solutions
                .iter_mut()
                .enumerate()
                .zip(inputs)
                .for_each(|((day, solution), input)| match input {
                    Ok(input) => {
                        println!(
                            "Day {} part 1: {}\nDay {} part 2: {}",
                            day + 1,
                            solution.part1(&input),
                            day + 1,
                            solution.part2(&input)
                        );
                    }
                    Err(_) => println!("Input not found for day {}", day + 1),
                });
        }
    });
}
