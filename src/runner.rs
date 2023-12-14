use ascii_table::{Align, AsciiTable};
use core::fmt;
use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};

use rayon::prelude::*;

use crate::solutions::{get_parser_fns, ParserFn};

#[derive(Debug)]
struct DayResult {
    parse: Duration,
    part1_duration: Option<Duration>,
    part2_duration: Option<Duration>,
    part1_result: Option<String>,
    part2_result: Option<String>,
}

#[derive(Debug)]
struct CompleteDayResult {
    day: usize,
    io_duration: Duration,
    day_result: DayResult,
}

struct Totals {
    io: Duration,
    parse: Duration,
    part1: Duration,
    part2: Duration,
}

impl Totals {
    fn new() -> Self {
        Self {
            io: Duration::ZERO,
            parse: Duration::ZERO,
            part1: Duration::ZERO,
            part2: Duration::ZERO,
        }
    }
}

pub fn run_days(days: &[usize], part: Option<i64>, input_dir: &Path) {
    let solutions = get_parser_fns();

    let results = days
        .par_iter()
        .filter_map(|&day| {
            if solutions.len() >= day {
                let start = Instant::now();
                if let Ok(input) = get_input_for_day(input_dir, day) {
                    let io_duration = Instant::now().duration_since(start);
                    Some(CompleteDayResult {
                        day,
                        io_duration,
                        day_result: run_day(solutions[day - 1], part, input),
                    })
                } else {
                    println!("Couldn't open input file for day {}", day);
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let totals = results
        .iter()
        .fold(Totals::new(), |mut totals, complete_day_result| {
            totals.io += complete_day_result.io_duration;
            totals.parse += complete_day_result.day_result.parse;
            if let Some(part1) = complete_day_result.day_result.part1_duration {
                totals.part1 += part1;
            }
            if let Some(part2) = complete_day_result.day_result.part2_duration {
                totals.part2 += part2;
            }
            totals
        });

    print_results(results, totals);
}

fn run_day(solution: ParserFn, part: Option<i64>, input: String) -> DayResult {
    let parse_start = Instant::now();
    let mut solution = solution(input);
    let parse = Instant::now().duration_since(parse_start);

    let (part1_duration, part1_result, part2_duration, part2_result) = match part {
        Some(1) => {
            let start = Instant::now();
            let result = solution.part1();
            let duration = Instant::now().duration_since(start);
            (Some(duration), Some(result), None, None)
        }
        Some(2) => {
            let start = Instant::now();
            let result = solution.part2();
            let duration = Instant::now().duration_since(start);
            (None, None, Some(duration), Some(result))
        }
        None => {
            let part1_start = Instant::now();
            let part1_result = solution.part1();
            let part1_duration = Instant::now().duration_since(part1_start);
            let part2_start = Instant::now();
            let part2_result = solution.part2();
            let part2_duration = Instant::now().duration_since(part2_start);
            (
                Some(part1_duration),
                Some(part1_result),
                Some(part2_duration),
                Some(part2_result),
            )
        }
        _ => panic!(),
    };
    DayResult {
        parse,
        part1_duration,
        part2_duration,
        part1_result,
        part2_result,
    }
}

fn print_results(results: Vec<CompleteDayResult>, totals: Totals) {
    if results.is_empty() {
        return;
    }
    let totals = if results.len() > 1 {
        vec![[
            "Total".to_owned(),
            format!("{:?}", totals.io),
            format!("{:?}", totals.parse),
            format!("{:?}", totals.part1),
            format!("{:?}", totals.part2),
            format!(
                "{:?}",
                totals.io + totals.parse + totals.part1 + totals.part2
            ),
            "-".to_owned(),
            "-".to_owned(),
        ]]
    } else {
        vec![]
    };

    let results = results
        .into_iter()
        .map(|complete_day_result| {
            let day = complete_day_result.day.to_string();
            let io = format!("{:?}", complete_day_result.io_duration);
            let day_result = complete_day_result.day_result;
            let parse = format!("{:?}", day_result.parse);
            let part1_duration = option_debug_to_string(day_result.part1_duration);
            let part2_duration = option_debug_to_string(day_result.part2_duration);
            let total_duration = option_debug_to_string(Some(
                complete_day_result.io_duration
                    + day_result.parse
                    + day_result.part1_duration.unwrap_or(Duration::ZERO)
                    + day_result.part2_duration.unwrap_or(Duration::ZERO),
            ));
            let part1_result = option_display_to_string(day_result.part1_result);
            let part2_result = option_display_to_string(day_result.part2_result);
            [
                day,
                io,
                parse,
                part1_duration,
                part2_duration,
                total_duration,
                part1_result,
                part2_result,
            ]
        })
        .chain(totals)
        .collect::<Vec<_>>();

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(128);
    ascii_table
        .column(0)
        .set_header("Day")
        .set_align(Align::Right);
    ascii_table
        .column(1)
        .set_header("File IO")
        .set_align(Align::Right);
    ascii_table
        .column(2)
        .set_header("Parse")
        .set_align(Align::Right);
    ascii_table
        .column(3)
        .set_header("Part 1")
        .set_align(Align::Right);
    ascii_table
        .column(4)
        .set_header("Part 2")
        .set_align(Align::Right);
    ascii_table
        .column(5)
        .set_header("Total")
        .set_align(Align::Right);
    ascii_table
        .column(6)
        .set_header("Part 1 Result")
        .set_align(Align::Right);
    ascii_table
        .column(7)
        .set_header("Part 2 Result")
        .set_align(Align::Right);
    ascii_table.print(results);
}

fn option_debug_to_string<T: fmt::Debug>(t: Option<T>) -> String {
    match t {
        Some(t) => format!("{:?}", t),
        None => "-".to_string(),
    }
}

fn option_display_to_string<T: fmt::Display>(t: Option<T>) -> String {
    match t {
        Some(t) => format!("{}", t),
        None => "-".to_string(),
    }
}

fn get_input_for_day(dir: &Path, day: usize) -> Result<String, ()> {
    fs::read_to_string(dir.join(format!("{day}.txt"))).map_err(|_err| ())
}
