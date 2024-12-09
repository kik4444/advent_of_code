use itertools::Itertools;

fn differences(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).filter(|(c1, c2)| *c1 != *c2).count()
}

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

fn find_vertical_rows_smudge<T: AsRef<str>>(input: &[T]) -> Option<usize> {
    let points = input
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line1), (_, line2))| differences(line1.as_ref(), line2.as_ref()) < 2)
        .map(|((point, _), _)| std::cmp::min(point, input.len() - 1))
        .collect_vec();

    let mut result = 0;

    'outer: for point in points {
        for i in 1..=point {
            if point + i < input.len() - 1 && differences(input[point - i].as_ref(), input[point + i + 1].as_ref()) > 1 {
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
fn find_horizontal_columns_smudge(input: &[&str]) -> Option<usize> {
    let mut transposed = vec![];
    for x in 0..input[0].len() {
        let mut s = String::new();
        for line in input.iter() {
            s.push_str(&line[x..=x])
        }
        transposed.push(s);
    }

    find_vertical_rows_smudge(&transposed)
}

fn main() {
    let groups = include_str!("./sample.txt")
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .map(|(_key, group)| group.collect_vec())
        .filter(|group| !group.iter().all(|line| line.is_empty()))
        .collect_vec();

    let res = groups
        .iter()
        .map(|group| {
            let first = if let Some(n) = find_vertical_rows(group) {
                Some(n * 100)
            } else {
                find_horizontal_columns(group)
            };

            let second = if let Some(n) = find_vertical_rows_smudge(group) {
                Some(n * 100)
            } else {
                find_horizontal_columns_smudge(group)
            };

            if first.is_some() && second.is_some() && first != second {
                second.unwrap_or_default()
            } else {
                first.unwrap_or_default()
            }
        })
        .inspect(|n| println!("{:#?}", n))
        .sum::<usize>();

    println!("{res}"); // 53360 too high
}
