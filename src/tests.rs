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

#[test]
fn day4() {
    let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    let result1 = "13";
    let result2 = "30";
    test_day(4, input, result1, input, result2);
}

#[test]
fn day5() {
    let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    let result1 = "35";
    let result2 = "46";
    test_day(5, input, result1, input, result2);
}

#[test]
fn day6() {
    let input = "\
Time:      7  15   30
Distance:  9  40  200
";
    let result1 = "288";
    let result2 = "71503";
    test_day(6, input, result1, input, result2);
}

#[test]
fn day7() {
    let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    let result1 = "6440";
    let result2 = "5905";
    test_day(7, input, result1, input, result2);
}
