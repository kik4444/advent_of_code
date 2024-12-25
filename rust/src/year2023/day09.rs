use itertools::Itertools;

pub fn part1(input: &str) {
    let res = parse(input)
        .iter()
        .map(|vec| vec.iter().rev().fold(0, |sum, sequence| sequence.last().unwrap() + sum))
        .sum::<isize>();

    println!("{res}");
}

pub fn part2(input: &str) {
    let res = parse(input)
        .iter()
        .map(|vec| vec.iter().rev().fold(0, |backward, sequence| sequence.first().unwrap() - backward))
        .sum::<isize>();

    println!("{res}");
}

fn parse(input: &str) -> Vec<Vec<Vec<isize>>> {
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
        .collect_vec()
}

fn differences(input: &[isize]) -> Vec<isize> {
    input.windows(2).map(|window| window[1] - window[0]).collect_vec()
}
