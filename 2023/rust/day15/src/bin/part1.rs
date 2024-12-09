use itertools::Itertools;

fn main() {
    let input = include_str!("./sample.txt").lines().flat_map(|line| line.split(',')).collect_vec();

    let res = input
        .iter()
        .map(|word| {
            word.chars().fold(0, |acc, ch| {
                let step1 = acc + ch as usize;
                let step2 = step1 * 17;
                step2 % 256
            })
        })
        .sum::<usize>();

    println!("{res}");
}
