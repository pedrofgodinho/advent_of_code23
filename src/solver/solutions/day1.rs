pub fn part1(input: &str) -> String {
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

pub fn part2(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut first = None;
            let mut last = 0;
            for i in 0..line.len() {
                let digit = if let Some(digit) = line.chars().nth(i).unwrap().to_digit(10) {
                    Some(digit as u8)
                } else {
                    get_digit(&line[i..])
                };
                if let Some(digit) = digit {
                    if first.is_none() {
                        first = Some(digit);
                    }
                    last = digit;
                }
            }
            first.unwrap() as u32 * 10 + last as u32
        })
        .sum();
    sum.to_string()
}

fn get_digit(string: &str) -> Option<u8> {
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
