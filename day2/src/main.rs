use std::{collections::HashSet, str::FromStr};

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    let sum: u64 = EXAMPLE
        .trim()
        .split(",")
        .map(|line| line.parse().unwrap())
        .map(|range: IdRange| {
            (range.0..=range.1).fold(0, |acc, id| if is_invalid(id) { acc + id } else { acc })
        })
        .sum();

    println!("day1: {sum}");

    let sum: u64 = EXAMPLE
        .split(",")
        .map(|line| line.parse().unwrap())
        .map(|range: IdRange| {
            (range.0..=range.1).fold(
                0,
                |acc, id| {
                    if is_invalid_part2(id) { acc + id } else { acc }
                },
            )
        })
        .sum();
    println!("day2: {sum}")
}

#[cfg(test)]
mod test {
    use crate::{IdRange, is_invalid, is_invalid_part2};

    #[test]
    fn should_parse_range() {
        assert_eq!("123-123".parse(), Ok(IdRange(123, 123)));
    }

    macro_rules! test_invalid {
        ($name:ident, $function:ident ,$input:expr) => {
            #[test]
            fn $name() {
                assert!($function($input))
            }
        };
    }

    test_invalid!(invalid_11, is_invalid, 11);
    test_invalid!(invalid_1212, is_invalid, 1212);
    test_invalid!(invalid_446446, is_invalid, 446446);

    test_invalid!(invalid_123123123, is_invalid_part2, 123123123);
    test_invalid!(invalid_11_again, is_invalid_part2, 11);
    test_invalid!(invalid_123123123123, is_invalid_part2, 123123123);
    test_invalid!(invalid_21, is_invalid_part2, 2121212121);
    test_invalid!(invalid_11111, is_invalid_part2, 11111);
    test_invalid!(invalid_999, is_invalid_part2, 999);
    test_invalid!(invalid_1188511885, is_invalid_part2, 1188511885);

    #[test]
    fn should_be_valid_part1() {
        assert!(!is_invalid(123));
    }

    macro_rules! test_valid {
        ($name:ident, $value:expr) => {
            #[test]
            fn $name() {
                assert!(!is_invalid_part2($value));
            }
        };
    }

    test_valid!(valid_1235, 1235);
    test_valid!(valid_12, 12);
    test_valid!(valid_123, 123);
    test_valid!(invalid_1188511880, 1188511880);
    test_valid!(invalid_11112, 11112);
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

fn is_invalid_part2(i: u64) -> bool {
    return search_pattern(i, 1);
}

fn search_pattern(i: u64, size: usize) -> bool {
    let stringified = i.to_string();
    let needle = &i.to_string()[..size];

    for window in ((size)..stringified.len()).step_by(size) {
        let Some(current_needle) = stringified.get(window..window + size) else {
            return false;
        };
        if needle == current_needle {
            continue;
        }
        return if size >= stringified.len().div_euclid(2) {
            false
        } else {
            search_pattern(i, size + 1)
        };
    }
    return true;
}
