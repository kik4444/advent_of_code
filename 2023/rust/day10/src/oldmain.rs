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
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn routes(&self, current: Point) -> Option<(Point, Point)> {
        match self {
            Pipe::NorthSouth => Some((Point { y: current.y - 1, ..current }, Point { y: current.y + 1, ..current })),
            Pipe::EastWest => Some((Point { x: current.x + 1, ..current }, Point { x: current.x - 1, ..current })),
            Pipe::NorthEast => Some((Point { y: current.y - 1, ..current }, Point { x: current.x + 1, ..current })),
            Pipe::NorthWest => Some((Point { y: current.y - 1, ..current }, Point { x: current.x - 1, ..current })),
            Pipe::SouthWest => Some((Point { y: current.y + 1, ..current }, Point { x: current.x - 1, ..current })),
            Pipe::SouthEast => Some((Point { y: current.y + 1, ..current }, Point { x: current.x + 1, ..current })),
            Pipe::Ground => None,
            Pipe::Start => unreachable!(),
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn infer_start(input: &[Vec<Pipe>], pos: Point) -> Pipe {
    let surroundings = [
        input[pos.x + 1][pos.y],
        input[pos.x][pos.y + 1],
        input[pos.x - 1][pos.y],
        input[pos.x][pos.y - 1],
        input[pos.x + 1][pos.y],
    ];

    let a = surroundings.windows(2).map(|window| (window[0], window[1])).collect_vec();
    println!("{:#?}", a);

    // match ().windows(2).collect_tuple().unwrap() {
    //     (a, b) => todo!(),
    // }

    panic!("Could not infer start")
}

fn main() {
    let input = include_str!("./sample.txt")
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect_vec())
        .collect_vec();

    let start_point = input
        .iter()
        .find_position_map(|line| line.iter().find_position(|p| **p == Pipe::Start))
        .map(|(x, (y, _))| Point { x, y })
        .unwrap();
    let start_type = infer_start(&input, start_point);

    input.iter().for_each(|a| println!("{:?}", a));

    println!("{:?}", start_point);
}
