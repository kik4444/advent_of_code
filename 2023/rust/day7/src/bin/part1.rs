use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn get_value(label: char) -> u8 {
            match label {
                '2'..='9' => label as u8 - b'0',
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            }
        }

        get_value(self.0).cmp(&get_value(other.0))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl Hand {
    fn kind(&self) -> HandKind {
        // Check how many times each card appears in the hand
        let uniques = self.0.iter().counts();

        match uniques.len() {
            1 => HandKind::Five,
            // If there are 2 unique types of cards in the hand, get the max number of identical cards
            2 => match uniques.values().max().unwrap() {
                4 => HandKind::Four,
                3 => HandKind::FullHouse,
                _ => unreachable!(),
            },
            3 => match uniques.values().max().unwrap() {
                3 => HandKind::Three,
                2 => HandKind::TwoPair,
                _ => unreachable!(),
            },
            4 => HandKind::OnePair,
            5 => HandKind::High,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<Vec<Card>> for Hand {
    type Error = ();

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        assert_eq!(value.len(), 5);

        let mut array = [Card('2'); 5];
        for (i, card) in value.into_iter().enumerate() {
            array[i] = card;
        }

        Ok(Hand(array))
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().map(Card).collect_vec().try_into()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind_compare = self.kind().cmp(&other.kind());

        if kind_compare == std::cmp::Ordering::Equal {
            self.0.cmp(&other.0)
        } else {
            kind_compare
        }
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: usize,
}

fn main() {
    let input = include_str!("./input.txt");
    let res = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| Game {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
        .sorted_by_key(|c| c.hand)
        .enumerate()
        .map(|(rank, game)| game.bid * (rank + 1))
        .sum::<usize>();

    println!("{:?}", res); // 247815719
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_cards() {
        assert_eq!(
            ['7', '5', '3', '8', '9', 'J', 'A', 'T', 'Q', 'K']
                .into_iter()
                .map(Card)
                .sorted()
                .collect_vec(),
            ['3', '5', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'].into_iter().map(Card).collect_vec()
        );
    }

    #[test]
    fn test_kind() {
        assert_eq!("AAAAA".parse::<Hand>().unwrap().kind(), HandKind::Five);
        assert_eq!("AA8AA".parse::<Hand>().unwrap().kind(), HandKind::Four);
        assert_eq!("23332".parse::<Hand>().unwrap().kind(), HandKind::FullHouse);
        assert_eq!("TTT98".parse::<Hand>().unwrap().kind(), HandKind::Three);
        assert_eq!("23432".parse::<Hand>().unwrap().kind(), HandKind::TwoPair);
        assert_eq!("A23A4".parse::<Hand>().unwrap().kind(), HandKind::OnePair);
        assert_eq!("23456".parse::<Hand>().unwrap().kind(), HandKind::High);
    }

    #[test]
    fn order_hands() {
        let mut source = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].map(|h| h.parse::<Hand>().unwrap());
        let target = ["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"].map(|h| h.parse::<Hand>().unwrap());

        source.sort();

        assert_eq!(source, target);
    }
}
