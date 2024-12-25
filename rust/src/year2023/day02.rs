pub fn part1(input: &str) {
    let res = input.lines().filter_map(valid_line).sum::<usize>();
    println!("{res}");
}

pub fn part2(input: &str) {
    let res = input.lines().map(find_power).sum::<usize>();
    println!("{res}");
}

fn valid_line(line: &str) -> Option<usize> {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

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
