use super::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn part1(&self, input: &str) -> String {
        let red = 12;
        let green = 13;
        let blue = 14;

        input
            .lines()
            .map(Game::from_str)
            .filter(|game| {
                game.groups
                    .iter()
                    .all(|group| group.possible_for(red, green, blue))
            })
            .map(|game| game.id)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(Game::from_str)
            .map(|game| game.mins())
            .map(|min| min.red * min.blue * min.green)
            .sum::<usize>()
            .to_string()
    }

    fn setup(&mut self) {}
}

#[derive(Debug)]
struct Group {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    groups: Vec<Group>,
}

impl Group {
    fn possible_for(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

impl Game {
    fn from_str(string: &str) -> Self {
        let mut sections = string.split(": ");
        let id = sections
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let groups = sections
            .next()
            .unwrap()
            .split("; ")
            .map(|group| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for draw in group.split(", ") {
                    let mut split = draw.split(' ');
                    let number = split.next().unwrap().parse::<usize>().unwrap();
                    let color = split.next().unwrap();
                    match color {
                        "red" => red = number,
                        "green" => green = number,
                        "blue" => blue = number,
                        _ => (),
                    }
                }
                Group { red, green, blue }
            })
            .collect::<Vec<_>>();

        Self { id, groups }
    }

    fn mins(&self) -> Group {
        let red = self.groups.iter().map(|group| group.red).max().unwrap();
        let blue = self.groups.iter().map(|group| group.blue).max().unwrap();
        let green = self.groups.iter().map(|group| group.green).max().unwrap();
        Group { red, green, blue }
    }
}
