use std::{error::Error, str::FromStr};

use aoc_runner_derive::*;

enum L {
    A,
    B,
    C,
}

enum R {
    X,
    Y,
    Z,
}

pub struct Round(L, R);

impl Round {
    fn score(&self) -> u32 {
        let mut score = 0;

        match self.1 {
            R::X => score += 1,
            R::Y => score += 2,
            R::Z => score += 3,
        }

        match self {
            Round(L::A, R::Z) | Round(L::B, R::X) | Round(L::C, R::Y) => score += 0,
            Round(L::A, R::X) | Round(L::B, R::Y) | Round(L::C, R::Z) => score += 3,
            Round(L::A, R::Y) | Round(L::B, R::Z) | Round(L::C, R::X) => score += 6,
        }

        score
    }

    fn to_result(&self) -> Self {
        match self.1 {
            R::X => match self.0 {
                L::A => Round(L::A, R::Z),
                L::B => Round(L::B, R::X),
                L::C => Round(L::C, R::Y),
            }, // LOSE
            R::Y => match self.0 {
                L::A => Round(L::A, R::X),
                L::B => Round(L::B, R::Y),
                L::C => Round(L::C, R::Z),
            }, // DRAW
            R::Z => match self.0 {
                L::A => Round(L::A, R::Y),
                L::B => Round(L::B, R::Z),
                L::C => Round(L::C, R::X),
            }, // WIN
        }
    }
}

impl FromStr for Round {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let l = match chars.next().unwrap() {
            'A' => L::A,
            'B' => L::B,
            'C' => L::C,
            _ => unreachable!(),
        };

        chars.next();

        let r = match chars.next().unwrap() {
            'X' => R::X,
            'Y' => R::Y,
            'Z' => R::Z,
            _ => unreachable!(),
        };

        Ok(Round(l, r))
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Round> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Round]) -> u32 {
    input.iter().map(|round| round.score()).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Round]) -> u32 {
    input.iter().map(|round| round.to_result().score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part2(&input), 12);
    }
}
