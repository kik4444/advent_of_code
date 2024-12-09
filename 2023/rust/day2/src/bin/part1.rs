const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn valid_line(line: &str) -> Option<usize> {
    let (beginning, rest) = line.split_once(": ").unwrap();
    let id = beginning[4..].trim().parse().unwrap();

    for set in rest.split("; ") {
        for count in set.split(", ") {
            let (num, cube) = count.split_once(' ').unwrap();
            let parsed_num = num.parse::<usize>().unwrap();
            match cube {
                "red" if parsed_num > MAX_RED => return None,
                "green" if parsed_num > MAX_GREEN => return None,
                "blue" if parsed_num > MAX_BLUE => return None,
                _ => continue,
            }
        }
    }

    Some(id)
}

fn main() {
    let input = include_str!("./input.txt");
    let res = input.lines().filter_map(valid_line).sum::<usize>();
    println!("{res}"); // 2600
}
