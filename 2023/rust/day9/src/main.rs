use itertools::Itertools;

fn differences(input: &[isize]) -> Vec<isize> {
    input.windows(2).map(|window| window[1] - window[0]).collect_vec()
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect_vec())
        .map(|seq| vec![seq])
        .map(|mut vec| loop {
            let new = differences(vec.last().unwrap());
            vec.push(new);
            if vec.last().unwrap().iter().all(|n| *n == 0) {
                break vec;
            }
        })
        .map(|vec| vec.iter().rev().fold(0, |sum, sequence| sequence.last().unwrap() + sum))
        .sum::<isize>()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect_vec())
        .map(|seq| vec![seq])
        .map(|mut vec| loop {
            let new = differences(vec.last().unwrap());
            vec.push(new);
            if vec.last().unwrap().iter().all(|n| *n == 0) {
                break vec;
            }
        })
        .map(|vec| vec.iter().rev().fold(0, |backward, sequence| sequence.first().unwrap() - backward))
        .sum::<isize>()
}

fn main() {
    let input = include_str!("./input.txt");
    let part1_res = part1(input);
    let part2_res = part2(input);

    println!("{part1_res}"); // 2174807968
    println!("{part2_res}"); // 1208
}
