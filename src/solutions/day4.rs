use super::Solution;

pub struct Day4 {}

impl Solution for Day4 {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(Card::parse)
            .map(|card| card.value())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let cards: Vec<_> = input.lines().map(Card::parse).collect();
        let mut counts = Vec::new();
        counts.resize(cards.len(), 1);

        for card in cards {
            let matches = card.match_count();
            let card_count = counts[card.id];
            for i in 1..=matches {
                if card.id + i >= counts.len() {
                    break;
                }
                counts[card.id + i] += card_count;
            }
        }

        counts.iter().sum::<usize>().to_string()
    }

    fn setup(&mut self) {}
}

impl Day4 {
    pub fn new() -> Self {
        Day4 {}
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    has: Vec<usize>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut split = line.split(": ");
        let id = split
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;
        let mut parts = split.next().unwrap().split(" | ");

        Self {
            id,
            winning: parts
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|number| number.parse().unwrap())
                .collect(),
            has: parts
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|number| number.parse().unwrap())
                .collect(),
        }
    }

    fn match_count(&self) -> usize {
        self.has.iter().filter(|n| self.winning.contains(n)).count()
    }

    fn value(&self) -> usize {
        let count: usize = self.has.iter().filter(|n| self.winning.contains(n)).count();
        if count == 0 {
            return 0;
        }
        2_usize.pow((count - 1) as u32)
    }
}
