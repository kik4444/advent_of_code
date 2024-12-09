fn get_number(input: &str) -> Option<usize> {
    let first = input.chars().find(|c| c.is_ascii_digit())?;
    let second = input.chars().rev().find(|c| c.is_ascii_digit())?;

    const ZERO: usize = '0' as usize;
    Some((first as usize - ZERO) * 10 + (second as usize - ZERO))
}

fn main() {
    let input = include_str!("./input.txt");
    let res = input.lines().filter_map(get_number).sum::<usize>();
    println!("{res}"); // 55090
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_once() {
        let num = get_number("164two");

        assert_eq!(num, Some(14));
    }
}
