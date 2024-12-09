fn find_power(line: &str) -> usize {
    let mut red = 1;
    let mut green = 1;
    let mut blue = 1;

    let (_, rest) = line.split_once(": ").unwrap();

    for set in rest.split("; ") {
        for cubes in set.split(", ") {
            let (num, cube) = cubes.split_once(' ').unwrap();
            let parsed_num = num.parse().unwrap();
            match cube {
                "red" if parsed_num > red => red = parsed_num,
                "green" if parsed_num > green => green = parsed_num,
                "blue" if parsed_num > blue => blue = parsed_num,
                _ => continue,
            }
        }
    }

    red * green * blue
}

fn main() {
    let input = include_str!("./input.txt");
    let res = input.lines().map(find_power).sum::<usize>();
    println!("{res}"); // 86036
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            find_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        )
    }
}
