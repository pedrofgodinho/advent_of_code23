use clap::Parser;
use std::path::PathBuf;

mod runner;
mod solutions;
#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Days to solve. Accepts ranges. For example: `1,3..10,13` solves 1, 3 through 10, and 13. Range is inclusive
    #[arg(short, long, default_value = "1..25", value_delimiter = ',', value_parser = parse_day_string)]
    days: Vec<String>,
    /// The part to solve. If not specified, both parts are solved.
    #[arg(short, long, value_parser=1..=2)]
    part: Option<i64>,
    /// Directory with the input files in format "{day}.txt".
    #[arg(short, long, default_value = "./in")]
    in_dir: PathBuf,
}

fn parse_day_string(str: &str) -> Result<String, String> {
    parse_day(str).map(|_| str.to_owned())
}

fn parse_day(str: &str) -> Result<Vec<usize>, String> {
    if str.contains("..") {
        let mut split = str.split("..");
        let mut left = split.next().unwrap();
        let mut right = split.next().unwrap();
        if left.is_empty() {
            left = "1";
        }
        if right.is_empty() {
            right = "25";
        }
        match (left.parse(), right.parse()) {
            (Ok(left), Ok(right)) => {
                if right > 25 || left == 0 || right < left {
                    Err(format!("Invalid range: {}", str))
                } else {
                    Ok((left..=right).collect())
                }
            }
            _ => Err(format!("Invalid range: {}", str)),
        }
    } else {
        if let Ok(day) = str.parse::<usize>() {
            if day > 25 || day == 0 {
                Err(format!("Invalid day: {}", str))
            } else {
                Ok(vec![day])
            }
        } else {
            Err(format!("Invalid day: {}", str))
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut days = args
        .days
        .iter()
        .flat_map(|day| parse_day(&day).unwrap())
        .collect::<Vec<_>>();
    days.sort();
    days.dedup();

    crate::runner::run_days(&days, args.part, &args.in_dir);
}
