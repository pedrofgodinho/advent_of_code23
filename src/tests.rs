use crate::solutions::get_solutions;

fn test_day(day: usize, input1: &str, result1: &str, input2: &str, result2: &str) {
    let mut solutions = get_solutions();
    let solution = &mut solutions[day - 1];
    solution.setup();

    assert_eq!(solution.part1(input1), result1);
    assert_eq!(solution.part2(input2), result2);
}

#[test]
fn day1() {
    let input1 = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    let result1 = "142";
    let input2 = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    let result2 = "281";
    test_day(1, input1, result1, input2, result2);
}

#[test]
fn day2() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    let result1 = "8";
    let result2 = "2286";
    test_day(2, input, result1, input, result2);
}

#[test]
fn day3() {
    let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    let result1 = "4361";
    let result2 = "467835";
    test_day(3, input, result1, input, result2);
}
