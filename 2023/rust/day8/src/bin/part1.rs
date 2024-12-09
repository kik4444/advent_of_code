use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Network<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Network<'a> {
    fn pick(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt").lines().collect_vec();
    let directions = input.iter().take(1).flat_map(|line| line.chars().map(Direction::from)).collect_vec();
    let nodes = input
        .into_iter()
        .skip(2)
        .map(|node| {
            let (value, network) = node.split_once(" = ").unwrap();
            let (left, right) = network[1..network.len() - 1].split_once(", ").unwrap();

            (value, Network { left, right })
        })
        .collect::<HashMap<_, _>>();

    let mut steps = 1_usize;
    let mut current = "AAA";

    for direction in directions.into_iter().cycle() {
        println!("now at {current}, step {steps}");
        current = nodes[current].pick(direction);
        if current == "ZZZ" {
            break;
        }
        steps += 1;
    }

    println!("{steps}"); // 21883
}
