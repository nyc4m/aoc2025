use std::str::FromStr;

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    let ranges: Vec<IdRange> = include_str!("input")
        .trim()
        .split(",")
        .map(|line| line.parse().unwrap())
        .collect();

    let mut sum = 0;
    for range in ranges {
        for i in range.0..=range.1 {
            sum += if is_invalid(i) { i } else { 0 };
        }
    }

    println!("{sum}");
}

#[cfg(test)]
mod test {
    use crate::{IdRange, is_invalid};

    #[test]
    fn should_parse_range() {
        assert_eq!("123-123".parse(), Ok(IdRange(123, 123)));
    }

    macro_rules! test_invalid {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                assert!(is_invalid($input))
            }
        };
    }

    test_invalid!(invalid_11, 11);
    test_invalid!(invalid_1212, 1212);
    test_invalid!(invalid_446446, 446446);

    #[test]
    fn should_be_valid() {
        assert!(!is_invalid(123));
    }
}

#[derive(Debug, PartialEq)]
struct IdRange(u64, u64);

impl FromStr for IdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();
        Ok(IdRange(
            start
                .parse()
                .expect(&format!("failed because of {}", start)),
            end.parse().expect(&format!("failed because of {}", start)),
        ))
    }
}

fn is_invalid(i: u64) -> bool {
    let stringified = i.to_string();
    if stringified.len() % 2 != 0 {
        return false;
    }
    &stringified[..stringified.len() / 2] == &stringified[stringified.len() / 2..]
}
