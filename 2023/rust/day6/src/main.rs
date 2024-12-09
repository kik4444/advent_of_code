use itertools::Itertools;

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

fn main() {
    let input = include_str!("./input.txt");
    let games_part1 = parse_part1(input);
    let games_part2 = parse_part2(input);

    for games in [games_part1, games_part2] {
        let res = games
            .into_iter()
            .map(|game| (1..game.time_ms).filter(|i| (game.time_ms - i) * i > game.distance_mm).count())
            .product::<usize>();

        println!("{res}"); // 211904
                           //43364472
    }
}
