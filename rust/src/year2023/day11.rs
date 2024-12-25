use itertools::Itertools;

pub fn part1(input: &str) {
    println!("{}", solve(input, 1));
}

pub fn part2(input: &str) {
    println!("{}", solve(input, 999_999));
}

fn solve(input: &str, expansion: usize) -> isize {
    let input = input.lines().collect_vec();

    let mut galaxies = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter(|(_, ch)| *ch == '#').map(move |(x, _)| Galaxy {
                x,
                y,
                found_x: x,
                found_y: y,
            })
        })
        .collect_vec();

    for (y, line) in input.iter().enumerate() {
        if line.chars().all(|ch| ch == '.') {
            for galaxy in galaxies.iter_mut().filter(|g| g.found_y > y) {
                galaxy.y += expansion;
            }
        }
    }

    for x in 0..input[0].len() {
        if input.iter().all(|line| &line[x..=x] == ".") {
            for galaxy in galaxies.iter_mut().filter(|g| g.found_x > x) {
                galaxy.x += expansion;
            }
        }
    }

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| (pair[1].x as isize - pair[0].x as isize).abs() + (pair[1].y as isize - pair[0].y as isize).abs())
        .sum::<isize>()
}

#[derive(Debug, PartialEq, Eq)]
struct Galaxy {
    x: usize,
    y: usize,
    found_x: usize,
    found_y: usize,
}
