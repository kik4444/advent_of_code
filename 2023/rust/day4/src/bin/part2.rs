use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Card {
    copies: usize,
    wins: usize,
}

fn main() {
    let input = include_str!("./input.txt");
    let re = Regex::new(r"Card +\d+: (.*?) \| (.*)").unwrap();
    let mut cards = input
        .lines()
        .flat_map(|line| re.captures_iter(line).map(|c| c.extract::<2>()))
        .map(|(_, [cards, winning])| (cards.split_whitespace().collect_vec(), winning.split_whitespace().collect_vec()))
        .map(|(cards, winning)| Card {
            copies: 1,
            wins: cards.iter().filter(|card| winning.contains(card)).count(),
        })
        .collect_vec();

    for i in 0..cards.len() {
        for j in 1..=cards[i].wins {
            let copies = cards[i].copies;

            if let Some(nth_card) = cards.get_mut(i + j) {
                nth_card.copies += copies;
            }
        }
    }

    println!("{}", cards.iter().map(|c| c.copies).sum::<usize>()); // 12648035
}
