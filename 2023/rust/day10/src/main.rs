use itertools::Itertools;

trait FindPositionMap: Iterator {
    fn find_position_map<B, F>(&mut self, mut f: F) -> Option<(usize, B)>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        for (index, elt) in self.enumerate() {
            if let Some(res) = f(elt) {
                return Some((index, res));
            }
        }
        None
    }
}
impl<T: Iterator> FindPositionMap for T {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

fn get_directions(pipe: char) -> Option<(Direction, Direction)> {
    use Direction::*;
    match pipe {
        '|' => Some((North, South)),
        '-' => Some((East, West)),
        'L' => Some((North, East)),
        'J' => Some((North, West)),
        '7' => Some((South, West)),
        'F' => Some((South, East)),
        '.' => None,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn infer_start_directions(input: &[Vec<char>], pos: Point) -> (Direction, Direction) {
    let surroundings = (
        input[pos.y][pos.x + 1],
        input[pos.y + 1][pos.x],
        input[pos.y][pos.x - 1],
        input[pos.y - 1][pos.x],
    );

    // match ((get_directions(surroundings.0), get_directions(surroundings.1))) {}

    panic!("cannot infer start")
}

fn main() {
    let input = include_str!("./sample.txt").lines().map(|line| line.chars().collect_vec()).collect_vec();
    let start_point = input
        .iter()
        .find_position_map(|line| line.iter().find_position(|ch| **ch == 'S'))
        .map(|(x, (y, _))| Point { x, y })
        .unwrap();
    // println!("{:#?}", start_point);

    let start_directions = infer_start_directions(&input, start_point);

    // let matrix = input.iter().map(|line|)
}
