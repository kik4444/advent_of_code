use itertools::Itertools;

fn hash(label: &str) -> usize {
    label.chars().fold(0, |acc, ch| {
        let step1 = acc + ch as usize;
        let step2 = step1 * 17;
        step2 % 256
    })
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

fn main() {
    let input = include_str!("./input.txt").lines().flat_map(|line| line.split(',')).collect_vec();

    let mut hashmap = (0..256).map(|_| Vec::<Lens>::new()).collect_vec();

    for elem in input {
        let (symbol_index, symbol) = elem.chars().enumerate().find(|(_, ch)| "=-".contains(*ch)).unwrap();
        let label = &elem[0..symbol_index];
        let hashmap_index = hash(label);

        match symbol {
            '-' => hashmap[hashmap_index].retain(|lens| lens.label != label),
            '=' => {
                let focal_length = elem[symbol_index + 1..].parse().unwrap();

                if let Some(lens) = hashmap[hashmap_index].iter_mut().find(|lens| lens.label == label) {
                    lens.focal_length = focal_length;
                } else {
                    hashmap[hashmap_index].push(Lens { label, focal_length })
                }
            }
            _ => unreachable!(),
        }
    }

    let res = hashmap
        .iter()
        .enumerate()
        .flat_map(|(box_num, lens_slots)| {
            lens_slots
                .iter()
                .enumerate()
                .map(move |(slot, lens)| (1 + box_num) * (1 + slot) * lens.focal_length)
        })
        .sum::<usize>();

    println!("{res}"); // 228508
}
