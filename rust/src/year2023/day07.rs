use itertools::Itertools;

pub fn part1(input: &str) {
    let res = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| part1::Game {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
        .sorted_by_key(|c| c.hand)
        .enumerate()
        .map(|(rank, game)| game.bid * (rank + 1))
        .sum::<usize>();

    println!("{:?}", res);
}

pub fn part2(input: &str) {
    let res = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| part2::Game {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
        .sorted_by_key(|c| c.hand)
        .enumerate()
        .map(|(rank, game)| game.bid * (rank + 1))
        .sum::<usize>();

    println!("{:?}", res);
}

mod part1 {
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
    pub struct Hand([Card; 5]);

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
    pub struct Game {
        pub hand: Hand,
        pub bid: usize,
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
}

mod part2 {
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
                    'J' => 1,
                    '2'..='9' => label as u8 - b'0',
                    'T' => 10,
                    'Q' => 11,
                    'K' => 12,
                    'A' => 13,
                    _ => unreachable!(),
                }
            }

            get_value(self.0).cmp(&get_value(other.0))
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Hand([Card; 5]);

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
            let mut uniques = self.0.iter().counts();

            if uniques.len() == 1 {
                // Avoids edge case if hand is "JJJJJ"
                return HandKind::Five;
            } else if let Some(jokers) = uniques.remove(&Card('J')) {
                // Get the number of jokers and sum them with the best card in the rest of the hand
                let most_repeated = uniques.values().max().unwrap();

                let best_card = uniques
                    .iter()
                    .filter(|(_card, repeated)| *repeated == most_repeated)
                    .map(|(card, _)| card)
                    .sorted()
                    .max()
                    .unwrap();

                uniques.entry(*best_card).and_modify(|v| *v += jokers);
            }

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
    pub struct Game {
        pub hand: Hand,
        pub bid: usize,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn order_cards() {
            assert_eq!(
                ['7', '5', '3', '8', '9', 'J', 'A', 'T', 'Q', 'K', '2']
                    .into_iter()
                    .map(Card)
                    .sorted()
                    .collect_vec(),
                ['J', '2', '3', '5', '7', '8', '9', 'T', 'Q', 'K', 'A']
                    .into_iter()
                    .map(Card)
                    .collect_vec()
            );
        }

        #[test]
        fn test_kind() {
            assert_eq!("AAJAA".parse::<Hand>().unwrap().kind(), HandKind::Five);
            assert_eq!("AA8JA".parse::<Hand>().unwrap().kind(), HandKind::Four);
            assert_eq!("23J32".parse::<Hand>().unwrap().kind(), HandKind::FullHouse);
            assert_eq!("TJT98".parse::<Hand>().unwrap().kind(), HandKind::Three);
            assert_eq!("23432".parse::<Hand>().unwrap().kind(), HandKind::TwoPair);
            assert_eq!("J2T3K".parse::<Hand>().unwrap().kind(), HandKind::OnePair);
            assert_eq!("23456".parse::<Hand>().unwrap().kind(), HandKind::High);
        }

        #[test]
        fn order_hands() {
            let mut source = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].map(|h| h.parse::<Hand>().unwrap());
            let target = ["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"].map(|h| h.parse::<Hand>().unwrap());

            source.sort();

            assert_eq!(source, target);
        }
    }
}
