use regex::Regex;

#[derive(Debug)]
struct Symbol {
    value: String,
    line: usize,
    pos: usize,
    numbers: Vec<usize>,
}

fn parse_around_num(line: &str, index: usize) -> Option<usize> {
    if !line.as_bytes()[index].is_ascii_digit() {
        return None;
    }

    let mut start = index;
    let mut end = index;

    while start > 0 && line.as_bytes()[start - 1].is_ascii_digit() {
        start -= 1;
    }
    while end < line.len() && line.as_bytes()[end].is_ascii_digit() {
        end += 1;
    }

    line[start..end].parse().ok()
}

fn populate_numbers(symbol: Symbol, input: &[&str]) -> Symbol {
    let mut numbers = vec![];

    if symbol.line > 0 {
        if let Some(num) = parse_around_num(input[symbol.line - 1], symbol.pos) {
            numbers.push(num)
        } else {
            if let Some(num) = parse_around_num(input[symbol.line - 1], symbol.pos - 1) {
                numbers.push(num)
            }
            if let Some(num) = parse_around_num(input[symbol.line - 1], symbol.pos + 1) {
                numbers.push(num)
            }
        }
    }

    if symbol.line < input.len() - 1 {
        if let Some(num) = parse_around_num(input[symbol.line + 1], symbol.pos) {
            numbers.push(num)
        } else {
            if let Some(num) = parse_around_num(input[symbol.line + 1], symbol.pos - 1) {
                numbers.push(num)
            }
            if let Some(num) = parse_around_num(input[symbol.line + 1], symbol.pos + 1) {
                numbers.push(num)
            }
        }
    }

    if let Some(num) = parse_around_num(input[symbol.line], symbol.pos - 1) {
        numbers.push(num);
    }
    if let Some(num) = parse_around_num(input[symbol.line], symbol.pos + 1) {
        numbers.push(num);
    }

    Symbol { numbers, ..symbol }
}

fn main() {
    let input = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let re = Regex::new(r"[^\.\d]").unwrap();
    let res = input
        .iter()
        .enumerate()
        .flat_map(|(line_num, line)| {
            re.find_iter(line).map(move |m| Symbol {
                value: m.as_str().into(),
                line: line_num,
                pos: m.range().start,
                numbers: vec![],
            })
        })
        .map(|s| populate_numbers(s, &input))
        .filter(|s| s.value.as_str() == "*" && s.numbers.len() == 2)
        .map(|s| s.numbers.iter().product::<usize>())
        .sum::<usize>();

    println!("{res}"); // 73074886
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_middle() {
        assert_eq!(parse_around_num("....354.....", 5), Some(354));
    }

    #[test]
    fn test_start() {
        assert_eq!(parse_around_num("320...@#^$*@^#$", 1), Some(320));
    }

    #[test]
    fn test_end() {
        assert_eq!(parse_around_num("...............908", 15), Some(908));
    }
}
