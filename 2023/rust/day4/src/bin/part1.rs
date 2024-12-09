use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let re = Regex::new(r"Card +\d+: (.*?) \| (.*)").unwrap();
    let sum = input
        .lines()
        .flat_map(|line| re.captures_iter(line).map(|c| c.extract::<2>()))
        .map(|(_, [cards, winning])| (cards.split_whitespace().collect_vec(), winning.split_whitespace().collect_vec()))
        .map(|(cards, winning)| cards.iter().filter(|card| winning.contains(card)).count())
        .map(|win_count| {
            let mut x = match win_count {
                0 => 0,
                _ => 1,
            };

            for _ in 1..win_count {
                x *= 2
            }

            x
        })
        .sum::<usize>();

    println!("sum {sum}");
}
