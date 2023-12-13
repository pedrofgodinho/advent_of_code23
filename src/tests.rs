use crate::solutions::get_solutions;
use indoc::indoc;

fn test_day(day: usize, input: &str, result: &str, part: Part) {
    let mut solutions = get_solutions();
    let solution = &mut solutions[day - 1];
    solution.setup();

    match part {
        Part::Part1 => assert_eq!(solution.part1(input), result),
        Part::Part2 => assert_eq!(solution.part2(input), result),
    }
}

enum Part {
    Part1,
    Part2,
}

#[test]
fn day01_part1() {
    let input1 = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
    test_day(1, input1, "142", Part::Part1);
}

#[test]
fn day01_part2() {
    let input2 = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
    test_day(1, input2, "281", Part::Part2);
}

const DAY02_INPUT: &str = indoc! {"
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"};

#[test]
fn day02_part1() {
    test_day(2, DAY02_INPUT, "8", Part::Part1);
}

#[test]
fn day02_part2() {
    test_day(2, DAY02_INPUT, "2286", Part::Part2);
}

const DAY03_INPUT: &str = indoc! {"
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
"};

#[test]
fn day03_part1() {
    test_day(3, DAY03_INPUT, "4361", Part::Part1);
}

#[test]
fn day03_part2() {
    test_day(3, DAY03_INPUT, "467835", Part::Part2);
}

const DAY04_INPUT: &str = indoc! {"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"};

#[test]
fn day04_part1() {
    test_day(4, DAY04_INPUT, "13", Part::Part1);
}

#[test]
fn day04_part2() {
    test_day(4, DAY04_INPUT, "30", Part::Part2);
}

const DAY05_INPUT: &str = indoc! {"
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
"};

#[test]
fn day05_part1() {
    test_day(5, DAY05_INPUT, "35", Part::Part1);
}

#[test]
fn day05_part2() {
    test_day(5, DAY05_INPUT, "46", Part::Part2);
}

const DAY06_INPUT: &str = indoc! {"
    Time:      7  15   30
    Distance:  9  40  200
"};

#[test]
fn day06_part1() {
    test_day(6, DAY06_INPUT, "288", Part::Part1);
}

#[test]
fn day06_part2() {
    test_day(6, DAY06_INPUT, "71503", Part::Part2);
}

const DAY07_INPUT: &str = indoc! {"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
"};

#[test]
fn day07_part1() {
    test_day(7, DAY07_INPUT, "6440", Part::Part1);
}

#[test]
fn day07_part2() {
    test_day(7, DAY07_INPUT, "5905", Part::Part2);
}

#[test]
fn day08_part1() {
    let input1 = indoc! {"\
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};
    test_day(8, input1, "2", Part::Part1);
}

#[test]
fn day08_part2() {
    let input2 = indoc! {"\
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
    test_day(8, input2, "6", Part::Part2);
}

const DAY09_INPUT: &str = indoc! {"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
"};

#[test]
fn day09_part1() {
    test_day(9, DAY09_INPUT, "114", Part::Part1);
}

#[test]
fn day09_part2() {
    test_day(9, DAY09_INPUT, "2", Part::Part2);
}

#[test]
fn day10_part1() {
    let input1_1 = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};
    let input1_2 = indoc! {"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "};
    test_day(10, input1_1, "4", Part::Part1);
    test_day(10, input1_2, "8", Part::Part1);
}

#[test]
fn day10_part2() {
    let input2_1 = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
    let input2_2 = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
    test_day(10, input2_1, "8", Part::Part2);
    test_day(10, input2_2, "10", Part::Part2);
}

const DAY11_INPUT: &str = indoc! {"
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
"};

#[test]
fn day11_part1() {
    test_day(11, DAY11_INPUT, "374", Part::Part1);
}

#[test]
fn day11_part2() {
    test_day(11, DAY11_INPUT, "82000210", Part::Part2);
}

const DAY12_INPUT: &str = indoc! {"
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
"};

#[test]
fn day12_part1() {
    test_day(12, DAY12_INPUT, "21", Part::Part1);
}

#[test]
fn day12_part2() {
    test_day(12, DAY12_INPUT, "525152", Part::Part2);
}

const DAY13_INPUT: &str = indoc! {"
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
"};

#[test]
fn day13_part1() {
    test_day(13, DAY13_INPUT, "405", Part::Part1);
}

#[test]
fn day13_part2() {
    test_day(13, DAY13_INPUT, "400", Part::Part2);
}
