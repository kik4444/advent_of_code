pub fn part1(input: &str) {
    let res = input.lines().filter_map(get_number).sum::<usize>();
    println!("{res}");
}

pub fn part2(input: &str) {
    let res = input.lines().map(parse_line).sum::<usize>();
    println!("{res}");
}

fn get_number(input: &str) -> Option<usize> {
    let first = input.chars().find(|c| c.is_ascii_digit())?;
    let second = input.chars().rev().find(|c| c.is_ascii_digit())?;

    const ZERO: usize = '0' as usize;
    Some((first as usize - ZERO) * 10 + (second as usize - ZERO))
}

const LOOKUP_TABLE: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_line(input: &str) -> usize {
    let mut first = 0;
    let mut second = 0;

    for i in 0..input.len() {
        if let Some(num) = get_number2(&input[i..], |s| s[0..=0].parse(), |s, w| s.starts_with(w)) {
            first = num;
            break;
        }
    }

    for i in (0..=input.len() - 1).rev() {
        if let Some(num) = get_number2(
            &input[..=i],
            |s| s[s.len() - 1..=s.len() - 1].parse(),
            |s, w| s.ends_with(w),
        ) {
            second = num;
            break;
        }
    }

    first * 10 + second
}

fn get_number2(
    line: &str,
    check_num: impl Fn(&str) -> Result<usize, std::num::ParseIntError>,
    check_word: impl Fn(&str, &str) -> bool,
) -> Option<usize> {
    if let Ok(num) = check_num(line) {
        return Some(num);
    }

    for (word, num) in LOOKUP_TABLE {
        if check_word(line, word) {
            return Some(num);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let num = get_number("164two");

        assert_eq!(num, Some(14));
    }

    #[test]
    fn test_part2() {
        let num = parse_line("164two");

        assert_eq!(num, 12);
    }
}
