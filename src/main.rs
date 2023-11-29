use clap::Parser;
use rayon::ThreadPoolBuilder;
use std::{path::PathBuf, time::Instant};

mod solver;

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
}

fn main() {
    let args = Args::parse();

    if args.parallel {
        let _ = ThreadPoolBuilder::new().build_global();
    }

    let start = Instant::now();

    match args.day {
        Some(day) => match solver::solve(day, &args.in_dir) {
            Ok(solution) => println!(
                "Day {} part 1: {}\nDay {} part 2: {}",
                day, solution.0, day, solution.1
            ),
            Err(e) => println!("Error - {}", e),
        },
        None => solver::solve_all_sequential(&args.in_dir, args.parallel),
    }

    let end = Instant::now();
    println!(
        "Execution time: {:.04}ms",
        end.duration_since(start).as_secs_f64() * 1000.0
    );
}
