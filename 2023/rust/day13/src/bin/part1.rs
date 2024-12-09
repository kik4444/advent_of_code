use itertools::Itertools;

fn find_vertical_rows<T: AsRef<str>>(input: &[T]) -> Option<usize> {
    let points = input
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line1), (_, line2))| line1.as_ref() == line2.as_ref())
        .map(|((point, _), _)| std::cmp::min(point, input.len() - 1))
        .collect_vec();

    let mut result = 0;

    'outer: for point in points {
        for i in 1..=point {
            if point + i < input.len() - 1 && input[point - i].as_ref() != input[point + i + 1].as_ref() {
                continue 'outer;
            }
        }
        result += point + 1;
    }

    if result == 0 {
        None
    } else {
        Some(result)
    }
}

fn find_horizontal_columns(input: &[&str]) -> Option<usize> {
    let mut transposed = vec![];
    for x in 0..input[0].len() {
        let mut s = String::new();
        for line in input.iter() {
            s.push_str(&line[x..=x])
        }
        transposed.push(s);
    }

    find_vertical_rows(&transposed)
}

fn main() {
    let groups = include_str!("./input.txt")
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .map(|(_key, group)| group.collect_vec())
        .filter(|group| !group.iter().all(|line| line.is_empty()))
        .collect_vec();

    let res = groups
        .iter()
        .map(|group| {
            if let Some(n) = find_vertical_rows(group) {
                n * 100
            } else if let Some(n) = find_horizontal_columns(group) {
                n
            } else {
                println!("{:#?}", group);
                0
            }
        })
        .sum::<usize>();

    println!("{res}"); // 37113
}
