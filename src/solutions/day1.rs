use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let mut iter = line.chars().filter_map(|digit| digit.to_digit(10));
                let first = iter.next().unwrap();
                let last = iter.last().unwrap_or(first);
                first * 10 + last
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let sum: u32 = input
            .lines()
            .map(|line| {
                let mut first = 0;
                for i in 0..line.len() {
                    let digit = get_digit(&line[i..]);
                    if let Some(digit) = digit {
                        first = digit;
                        break;
                    }
                }

                let mut last = 0;
                for i in (0..line.len()).rev() {
                    let digit = get_digit(&line[i..]);
                    if let Some(digit) = digit {
                        last = digit;
                        break;
                    }
                }
                first * 10 + last
            })
            .sum();
        sum.to_string()
    }

    fn setup(&mut self) {}
}

fn get_digit(string: &str) -> Option<u32> {
    if let Some(digit) = string.chars().next().unwrap().to_digit(10) {
        return Some(digit);
    }
    if string.starts_with("one") {
        Some(1)
    } else if string.starts_with("two") {
        Some(2)
    } else if string.starts_with("three") {
        Some(3)
    } else if string.starts_with("four") {
        Some(4)
    } else if string.starts_with("five") {
        Some(5)
    } else if string.starts_with("six") {
        Some(6)
    } else if string.starts_with("seven") {
        Some(7)
    } else if string.starts_with("eight") {
        Some(8)
    } else if string.starts_with("nine") {
        Some(9)
    } else if string.starts_with("zero") {
        Some(0)
    } else {
        None
    }
}
