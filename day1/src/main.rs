use std::{ops::Rem, str::FromStr};

const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

const INPUT: &str = include_str!("input");

fn main() {
    let rotations: Vec<Rotation> = EXAMPLE.lines().map(|line| line.parse().unwrap()).collect();

    let mut times_in_went_to_zero = 0;
    let mut state = 50;
    for rotation in rotations {
        state = rotate(state, rotation);
        if state == 0 {
            times_in_went_to_zero += 1;
        }
    }

    let rotations: Vec<Rotation> = EXAMPLE.lines().map(|line| line.parse().unwrap()).collect();

    let mut nb0 = 0;
    let mut state = 50;
    for rotation in rotations {
        let output = pass_by_0(state, rotation);
        state = output.0;
        nb0 += output.1;
    }

    println!("Part1: {}", times_in_went_to_zero);
    println!("Part2: {}", nb0);
}

fn rotate(state: i16, rotation: Rotation) -> i16 {
    match rotation {
        Rotation::L(amount) => state - amount,
        Rotation::R(amount) => state + amount,
    }
    .rem_euclid(100)
}

#[derive(Debug, PartialEq)]
enum Rotation {
    R(i16),
    L(i16),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.get(1..).unwrap().parse().unwrap();
        if s.get(0..1) == Some("L") {
            Ok(Rotation::L(value))
        } else {
            Ok(Rotation::R(value))
        }
    }
}

fn pass_by_0(state: i16, rotation: Rotation) -> (i16, i16) {
    match rotation {
        Rotation::R(value) => {
            let new_state = state + value;
            (new_state.rem(100), new_state.div_euclid(100))
        }
        Rotation::L(value) => {
            let new_state = state - value;
            if new_state == 0 {
                return (new_state, 1 + new_state.div_euclid(-100));
            }
            if state == 0 {
                return (new_state.rem_euclid(100), new_state.div_euclid(-100) - 1);
            }
            (new_state.rem_euclid(100), new_state.div_euclid(-100))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Rotation, pass_by_0, rotate};

    #[test]
    fn should_rotate_to_left() {
        assert_eq!(rotate(50, Rotation::L(1)), 49);
    }

    #[test]
    fn should_rotate_to_right() {
        assert_eq!(rotate(50, Rotation::R(1)), 51);
    }

    #[test]
    fn should_rotate_to_right_and_be_0() {
        assert_eq!(rotate(99, Rotation::R(1)), 0);
    }

    #[test]
    fn should_rotate_to_left_and_be_99() {
        assert_eq!(rotate(0, Rotation::L(1)), 99);
    }

    #[test]
    fn should_rotate_to_left_and_be_98() {
        assert_eq!(rotate(0, Rotation::L(2)), 98);
    }

    #[test]
    fn should_parse_l() {
        assert_eq!("L1".parse(), Ok(Rotation::L(1)))
    }

    #[test]
    fn should_parse_r() {
        assert_eq!("R1".parse(), Ok(Rotation::R(1)))
    }

    #[test]
    fn should_pass_by_zero() {
        assert_eq!(pass_by_0(50, Rotation::L(68)), (82, 1));
        assert_eq!(pass_by_0(50, Rotation::R(150)), (0, 2));
        assert_eq!(pass_by_0(14, Rotation::L(82)), (32, 1));
        assert_eq!(pass_by_0(50, Rotation::R(1000)), (50, 10));
        assert_eq!(pass_by_0(50, Rotation::L(1000)), (50, 10));
        assert_eq!(pass_by_0(50, Rotation::R(50)), (0, 1));
        assert_eq!(pass_by_0(50, Rotation::R(50)), (0, 1));
    }

    #[test]
    fn should_not_pass_by_zero() {
        assert_eq!(pass_by_0(82, Rotation::L(30)), (52, 0));
    }

    #[test]
    fn test() {
        assert_eq!(pass_by_0(50, Rotation::L(50)), (0, 1));
        assert_eq!(pass_by_0(0, Rotation::L(1)), (99, 0));
        assert_eq!(pass_by_0(0, Rotation::L(100)), (0, 1));
        assert_eq!(pass_by_0(0, Rotation::R(100)), (0, 1))
    }
}
