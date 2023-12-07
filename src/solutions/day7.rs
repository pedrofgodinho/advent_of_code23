use super::Solution;
use itertools::Itertools;

pub struct Day7 {}

impl Solution for Day7 {
    fn part1(&self, input: &str) -> String {
        let mut hands = input
            .lines()
            .map(|line| Hand::from_line(line, false))
            .collect::<Vec<_>>();
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut hands = input
            .lines()
            .map(|line| Hand::from_line(line, true))
            .collect::<Vec<_>>();
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum::<usize>()
            .to_string()
    }

    fn setup(&mut self) {}
}

impl Day7 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
    part_2_rules: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Card {
    fn from_byte(byte: &u8) -> Option<Self> {
        match byte {
            b'A' => Some(Self::Ace),
            b'K' => Some(Self::King),
            b'Q' => Some(Self::Queen),
            b'J' => Some(Self::Jack),
            b'T' => Some(Self::Ten),
            b'9' => Some(Self::Nine),
            b'8' => Some(Self::Eight),
            b'7' => Some(Self::Seven),
            b'6' => Some(Self::Six),
            b'5' => Some(Self::Five),
            b'4' => Some(Self::Four),
            b'3' => Some(Self::Three),
            b'2' => Some(Self::Two),
            _ => None,
        }
    }

    fn value_2(&self) -> usize {
        match self {
            Card::Ace => 13,
            Card::King => 12,
            Card::Queen => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Jack => 1,
        }
    }

    fn compare_2(&self, other: &Self) -> std::cmp::Ordering {
        self.value_2().cmp(&other.value_2())
    }
}

impl Hand {
    fn from_line(line: &str, part_2: bool) -> Self {
        let mut parts = line.split_whitespace();
        let cards = parts
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .flat_map(Card::from_byte)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            cards,
            hand_type: if part_2 {
                Self::hand_type_2(&cards)
            } else {
                Self::hand_type_1(&cards)
            },
            bid: parts.next().unwrap().parse().unwrap(),
            part_2_rules: part_2,
        }
    }

    fn hand_type_1(my_cards: &[Card; 5]) -> HandType {
        let cards = my_cards.iter().unique().collect::<Vec<_>>();
        let first = my_cards[0];
        match cards.len() {
            5 => HandType::HighCard,
            4 => HandType::Pair,
            3 => {
                // 3 kind or 2 pair
                let check = cards[0];
                match my_cards.iter().filter(|&e| e == check).count() {
                    2 => HandType::TwoPairs,
                    3 => HandType::ThreeKind,
                    1 => {
                        let check = cards[1];
                        match my_cards.iter().filter(|&e| e == check).count() {
                            2 => HandType::TwoPairs,
                            1 | 3 => HandType::ThreeKind,
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            2 => {
                // full house or four kind
                match my_cards.iter().filter(|&&e| e == first).count() {
                    4 | 1 => HandType::FourKind,
                    3 | 2 => HandType::FullHouse,
                    _ => unreachable!(),
                }
            }
            1 => HandType::FiveKind,
            _ => unreachable!(),
        }
    }

    fn hand_type_2(my_cards: &[Card; 5]) -> HandType {
        let cards = my_cards
            .iter()
            .filter(|&&card| card != Card::Jack)
            .copied()
            .collect::<Vec<_>>();
        let mut max_count = 0;
        let mut max_card = if cards.len() > 0 { cards[0] } else { Card::Two };
        for card in &cards {
            if card == &Card::Jack {
                continue;
            }
            let count = cards.iter().filter(|&iter_card| iter_card == card).count();
            if count > max_count {
                max_count = count;
                max_card = *card;
            }
        }
        let mut my_cards = my_cards.clone();
        for card in my_cards.iter_mut() {
            if card == &Card::Jack {
                *card = max_card;
            }
        }
        Self::hand_type_1(&my_cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type == other.hand_type {
            for idx in 0..5 {
                let cmp = if self.part_2_rules {
                    self.cards[idx].compare_2(&other.cards[idx])
                } else {
                    self.cards[idx].cmp(&other.cards[idx])
                };
                if cmp.is_ne() {
                    return Some(cmp);
                }
            }
            panic!("Equal Hands");
        }
        Some(self.hand_type.cmp(&other.hand_type))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
