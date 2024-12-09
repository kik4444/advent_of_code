use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    Cubed,
    Rounded,
    Space,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Cubed,
            'O' => Self::Rounded,
            '.' => Self::Space,
            _ => unreachable!(),
        }
    }
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<T>>())
        .collect()
}

fn main() {
    let input = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().map(Rock::from).collect_vec())
        .collect_vec();

    let transposed = transpose2(input);

    let tilted = transposed
        .iter()
        .map(|rocks| {
            rocks
                .split(|rock| *rock == Rock::Cubed)
                .intersperse(&[Rock::Cubed])
                .flat_map(|rocks| rocks.iter().sorted())
                .collect_vec()
        })
        .collect_vec();

    let original = transpose2(tilted);

    let res = original
        .iter()
        .rev()
        .enumerate()
        .map(|(load, rocks)| rocks.iter().filter(|rock| ***rock == Rock::Rounded).count() * (load + 1))
        .sum::<usize>();

    println!("{res}"); // 109098
}
