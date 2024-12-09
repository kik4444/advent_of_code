use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};
use num_integer::Integer;

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

#[derive(Debug)]
struct Position<'a>(&'a str);

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

    let positions = nodes.keys().filter(|k| k.ends_with('A')).map(|position| Position(position)).collect_vec();

    let steps_per_position = positions
        .iter()
        .map(|pos| {
            directions
                .iter()
                .cycle()
                .fold_while((pos.0, 0_usize), |(current_pos, steps), dir| {
                    if current_pos.ends_with('Z') {
                        FoldWhile::Done((current_pos, steps))
                    } else {
                        FoldWhile::Continue((nodes[current_pos].pick(*dir), steps + 1))
                    }
                })
                .into_inner()
                .1
        })
        .collect_vec();

    let mut res = steps_per_position[0];
    for steps in steps_per_position[1..].iter() {
        res = res.lcm(steps)
    }

    println!("{res}");
}
