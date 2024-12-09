#[allow(dead_code)]
fn old_get_number(line: &str, range: impl Iterator<Item = usize>) -> usize {
    use libs::StrCharIndex;

    let mut result = 0;

    for i in range {
        if line.char_index(i).is_ascii_digit() {
            result = line.char_index(i) as usize - '0' as usize;
            break;
        } else {
            let remaining_len = line.len() - i;

            match line.char_index(i) {
                'o' if remaining_len >= 3 => {
                    if &line[i + 1..=i + 2] == "ne" {
                        result = 1;
                        break;
                    }
                }
                't' => {
                    if remaining_len >= 3 && &line[i + 1..=i + 2] == "wo" {
                        result = 2;
                        break;
                    } else if remaining_len >= 5 && &line[i + 1..=i + 4] == "hree" {
                        result = 3;
                        break;
                    }
                }
                'f' if remaining_len >= 4 => match &line[i + 1..=i + 3] {
                    "our" => {
                        result = 4;
                        break;
                    }
                    "ive" => {
                        result = 5;
                        break;
                    }
                    _ => (),
                },
                's' => {
                    if remaining_len >= 3 && &line[i + 1..=i + 2] == "ix" {
                        result = 6;
                        break;
                    } else if remaining_len >= 5 && &line[i + 1..=i + 4] == "even" {
                        result = 7;
                        break;
                    }
                }
                'e' if remaining_len >= 5 => {
                    if &line[i + 1..=i + 4] == "ight" {
                        result = 8;
                        break;
                    }
                }
                'n' if remaining_len >= 4 => {
                    if &line[i + 1..=i + 3] == "ine" {
                        result = 9;
                        break;
                    }
                }
                _ => continue,
            }
        }
    }

    result
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

fn new_get_number(
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

fn parse_line(input: &str) -> usize {
    // let first = old_get_number(input, 0..input.len());
    // let second = old_get_number(input, (0..=input.len() - 1).rev());

    let mut first = 0;
    let mut second = 0;

    for i in 0..input.len() {
        if let Some(num) = new_get_number(&input[i..], |s| s[0..=0].parse(), |s, w| s.starts_with(w)) {
            first = num;
            break;
        }
    }

    for i in (0..=input.len() - 1).rev() {
        if let Some(num) = new_get_number(&input[..=i], |s| s[s.len() - 1..=s.len() - 1].parse(), |s, w| s.ends_with(w)) {
            second = num;
            break;
        }
    }

    first * 10 + second
}

fn main() {
    let input = include_str!("./input.txt");
    let res = input.lines().map(parse_line).sum::<usize>();
    println!("{res}"); // 54845
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_once() {
        let num = parse_line("164two");

        assert_eq!(num, 12);
    }
}
