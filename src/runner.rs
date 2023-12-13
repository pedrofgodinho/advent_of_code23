use ascii_table::{Align, AsciiTable};
use core::fmt;
use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};

use crate::solutions::{get_parser_fns, ParserFn};

#[derive(Debug)]
struct DayResult {
    parse: Duration,
    part1_duration: Option<Duration>,
    part2_duration: Option<Duration>,
    part1_result: Option<String>,
    part2_result: Option<String>,
}

pub fn run_days(days: &[usize], part: Option<i64>, input_dir: &Path) {
    let solutions = get_parser_fns();
    let mut no_input = vec![];
    let mut no_solution = vec![];
    let results = days
        .iter()
        .filter_map(|&day| {
            if solutions.len() >= day {
                let start = Instant::now();
                if let Ok(input) = get_input_for_day(input_dir, day) {
                    let duration = Instant::now().duration_since(start);
                    Some((day, duration, run_day(solutions[day - 1], part, input)))
                } else {
                    no_input.push(day);
                    None
                }
            } else {
                no_solution.push(day);
                None
            }
        })
        .collect::<Vec<_>>();

    let totals = results.iter().fold(
        (
            Duration::ZERO,
            Duration::ZERO,
            Duration::ZERO,
            Duration::ZERO,
        ),
        |(io, parse, mut part1, mut part2), (_day, new_io, day_result)| {
            if let Some(new_part1) = day_result.part1_duration {
                part1 += new_part1;
            }
            if let Some(new_part2) = day_result.part2_duration {
                part2 += new_part2;
            }
            (io + *new_io, parse + day_result.parse, part1, part2)
        },
    );

    let totals = if results.len() > 1 {
        vec![[
            "Total".to_owned(),
            format!("{:?}", totals.0),
            format!("{:?}", totals.1),
            format!("{:?}", totals.2),
            format!("{:?}", totals.3),
            "-".to_owned(),
            "-".to_owned(),
        ]]
    } else {
        vec![]
    };
    let results = results
        .into_iter()
        .map(|(day, io_duration, day_result)| {
            let day = day.to_string();
            let io = format!("{:?}", io_duration);
            let parse = format!("{:?}", day_result.parse);
            let part1_duration = option_debug_to_string(day_result.part1_duration);
            let part2_duration = option_debug_to_string(day_result.part2_duration);
            let part1_result = option_display_to_string(day_result.part1_result);
            let part2_result = option_display_to_string(day_result.part2_result);
            [
                day,
                io,
                parse,
                part1_duration,
                part2_duration,
                part1_result,
                part2_result,
            ]
        })
        .chain(totals.into_iter())
        .collect::<Vec<_>>();

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(128);
    ascii_table
        .column(0)
        .set_header("Day")
        .set_align(Align::Right);
    ascii_table
        .column(1)
        .set_header("File IO Duration")
        .set_align(Align::Right);
    ascii_table
        .column(2)
        .set_header("Parse Duration")
        .set_align(Align::Right);
    ascii_table
        .column(3)
        .set_header("Part 1 Duration")
        .set_align(Align::Right);
    ascii_table
        .column(4)
        .set_header("Part 2 Duration")
        .set_align(Align::Right);
    ascii_table
        .column(5)
        .set_header("Part 1 Result")
        .set_align(Align::Right);
    ascii_table
        .column(6)
        .set_header("Part 2 Result")
        .set_align(Align::Right);
    ascii_table.print(results);

    for day in no_input {
        println!("Couldn't open input file for day {}", day);
    }
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
