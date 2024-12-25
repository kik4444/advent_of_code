use itertools::Itertools;

pub fn part1(input: &str) {
    // TODO
    let res = parse_part1(input)
        .into_iter()
        .map(|game| (1..game.time_ms).filter(|i| (game.time_ms - i) * i > game.distance_mm).count())
        .product::<usize>();

    println!("{res}");
}

pub fn part2(input: &str) {
    let res = parse_part2(input)
        .into_iter()
        .map(|game| (1..game.time_ms).filter(|i| (game.time_ms - i) * i > game.distance_mm).count())
        .product::<usize>();

    println!("{res}");
}

#[derive(Debug)]
struct Game {
    time_ms: usize,
    distance_mm: usize,
}

fn parse_part1(input: &str) -> Vec<Game> {
    let parsing = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|numbers| numbers.split_whitespace().map(|n| n.parse::<usize>().unwrap()));

    let times = parsing.clone().take(1).flatten().collect_vec();
    let distances = parsing.skip(1).flatten().collect_vec();

    times
        .into_iter()
        .zip(distances)
        .map(|(time_ms, distance_mm)| Game { time_ms, distance_mm })
        .collect_vec()
}

fn parse_part2(input: &str) -> Vec<Game> {
    let parsing = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|numbers| numbers.split_whitespace().join("").parse::<usize>().unwrap());

    let times = parsing.clone().take(1).collect_vec();
    let distances = parsing.skip(1).collect_vec();

    times
        .into_iter()
        .zip(distances)
        .map(|(time_ms, distance_mm)| Game { time_ms, distance_mm })
        .collect_vec()
}
