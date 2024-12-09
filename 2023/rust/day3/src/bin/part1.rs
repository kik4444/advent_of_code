use std::ops::Range;

use regex::Regex;

#[derive(Debug)]
struct Number {
    value: usize,
    line: usize,
    range: Range<usize>,
}

fn is_symbol(ch: impl Into<char>) -> bool {
    !matches!(ch.into(), '0'..='9' | '.')
}

pub trait GetAsBytes {
    fn get_as_bytes(&self, index: usize) -> Option<&[u8]>;
}

impl GetAsBytes for &[&str] {
    fn get_as_bytes(&self, index: usize) -> Option<&[u8]> {
        self.get(index).map(|i| i.as_bytes())
    }
}

/// Check 8 directions around the number's range
fn validate(number: Number, input: &[&str]) -> usize {
    let has_symbol = number
        .range
        .flat_map(|pos| {
            let ln = number.line;
            [
                ln.checked_sub(1).and_then(|ln| input.get_as_bytes(ln).and_then(|b| b.get(pos))),
                ln.checked_sub(1).and_then(|ln| input.get_as_bytes(ln).and_then(|b| b.get(pos + 1))),
                input[ln].as_bytes().get(pos + 1),
                input.get_as_bytes(ln + 1).and_then(|b| b.get(pos + 1)),
                input.get_as_bytes(ln + 1).and_then(|b| b.get(pos)),
                pos.checked_sub(1).and_then(|pos| input.get_as_bytes(ln + 1).and_then(|b| b.get(pos))),
                pos.checked_sub(1).and_then(|pos| input[ln].as_bytes().get(pos)),
                ln.checked_sub(1)
                    .and_then(|ln| pos.checked_sub(1).and_then(|pos| input.get_as_bytes(ln).and_then(|b| b.get(pos)))),
            ]
        })
        .flatten()
        .any(|ch| is_symbol(*ch));

    if has_symbol {
        number.value
    } else {
        0
    }
}

fn main() {
    let input = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let re = Regex::new(r"\d+").unwrap();
    let sum = input
        .iter()
        .enumerate()
        .flat_map(|(line_num, line)| {
            re.find_iter(line).map(move |m| Number {
                value: m.as_str().parse().unwrap(),
                line: line_num,
                range: m.range(),
            })
        })
        .map(|n| validate(n, &input))
        .sum::<usize>();

    println!("{sum}"); // 527369
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_symbol('@'))
    }
}
