use itertools::Itertools;
use regex::Regex;

pub fn part1(input: &str) {
    let res = parse(input)
        .iter()
        .map(|c| {
            let mut x = match c.wins {
                0 => 0,
                _ => 1,
            };

            for _ in 1..c.wins {
                x *= 2
            }

            x
        })
        .sum::<usize>();

    println!("{res}")
}

pub fn part2(input: &str) {
    let mut cards = parse(input);

    for i in 0..cards.len() {
        for j in 1..=cards[i].wins {
            let copies = cards[i].copies;

            if let Some(nth_card) = cards.get_mut(i + j) {
                nth_card.copies += copies;
            }
        }
    }

    println!("{}", cards.iter().map(|c| c.copies).sum::<usize>());
}

#[derive(Debug)]
struct Card {
    copies: usize,
    wins: usize,
}

fn parse(input: &str) -> Vec<Card> {
    let re = Regex::new(r"Card +\d+: (.*?) \| (.*)").unwrap();

    input
        .lines()
        .flat_map(|line| re.captures_iter(line).map(|c| c.extract::<2>()))
        .map(|(_, [cards, winning])| (cards.split_whitespace().collect_vec(), winning.split_whitespace().collect_vec()))
        .map(|(cards, winning)| Card {
            copies: 1,
            wins: cards.iter().filter(|card| winning.contains(card)).count(),
        })
        .collect_vec()
}
