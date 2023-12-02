use clap::Parser;
use rayon::{prelude::*, ThreadPoolBuilder};
use solutions::Solution;
use std::{fs, path::PathBuf, time::Instant};

use crate::solutions::get_solutions;

mod solutions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to solve. Do not specify to solve all challenges.
    #[arg(short, long)]
    day: Option<usize>,
    /// Directory with the input files in format "{day}.txt"
    #[arg(short, long, default_value = "./in")]
    in_dir: PathBuf,
    /// If flag is active, challenges will be solved in parallel. Only applies when solving all challenges.
    #[arg(short, long, action)]
    parallel: bool,
    /// Whether to include problem setup in timing (regex compiling, for instance)
    #[arg(short, long, action)]
    time_setup: bool,
}

fn get_input_for_day(dir: &PathBuf, day: usize) -> Result<String, ()> {
    fs::read_to_string(dir.join(format!("{day}.txt"))).map_err(|_err| ())
}

fn solve(solution: &Box<dyn Solution>, day: usize, in_dir: &PathBuf) {
    let input = match get_input_for_day(in_dir, day) {
        Ok(input) => input,
        Err(_) => {
            println!("Could not find input for day {day}");
            return;
        }
    };

    println!(
        "Day {} Part 1: {}\nDay {} Part 2: {}",
        day,
        solution.part1(&input),
        day,
        solution.part2(&input)
    );
}

fn main() {
    let args = Args::parse();

    let _ = ThreadPoolBuilder::new().build_global();

    let mut solutions = get_solutions();

    let start;

    match args.day {
        Some(day) => {
            // Solve one day only
            if day == 0 || day > solutions.len() {
                println!("Invalid day: {day}");
                return;
            }

            let solution = &mut solutions[day - 1];

            if args.time_setup {
                start = Instant::now();
                solution.setup();
            } else {
                solution.setup();
                start = Instant::now();
            }

            solve(solution, day, &args.in_dir);
        }
        None => {
            if !args.time_setup {
                solutions.iter_mut().for_each(|solution| solution.setup());
            }

            start = Instant::now();

            if args.parallel {
                solutions
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(day, solution)| {
                        if args.time_setup {
                            solution.setup();
                        }
                        solve(&solution, day + 1, &args.in_dir);
                    });
            } else {
                solutions
                    .iter_mut()
                    .enumerate()
                    .for_each(|(day, solution)| {
                        if args.time_setup {
                            solution.setup();
                        }
                        solve(solution, day + 1, &args.in_dir);
                    });
            }
        }
    }

    let end = Instant::now();
    println!(
        "Execution time: {:.04}ms",
        end.duration_since(start).as_secs_f64() * 1000.0
    );
}
