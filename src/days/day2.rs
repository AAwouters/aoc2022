use crate::util::util_imports::*;

#[derive(Clone, Copy)]
enum Symbol {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

impl FromStr for Symbol {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Symbol::A),
            "B" => Ok(Symbol::B),
            "C" => Ok(Symbol::C),
            "X" => Ok(Symbol::X),
            "Y" => Ok(Symbol::Y),
            "Z" => Ok(Symbol::Z),
            _ => Err(ParseError::new("Unknown symbol"))
        }
    }
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl From<Symbol> for RPS {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::A | Symbol::X => Self::Rock,
            Symbol::B | Symbol::Y => Self::Paper,
            Symbol::C | Symbol::Z => Self::Scissors,
        }
    }
}

impl From<(RPS, RPSResult)> for RPS {
    fn from(tuple: (RPS, RPSResult)) -> Self {
        let hand = tuple.0;
        let result = tuple.1;

        match hand {
            RPS::Rock => {
                match result {
                    RPSResult::Win => RPS::Paper,
                    RPSResult::Draw => RPS::Rock,
                    RPSResult::Lose => RPS::Scissors,
                }
            },
            RPS::Paper => {
                match result {
                    RPSResult::Win => RPS::Scissors,
                    RPSResult::Draw => RPS::Paper,
                    RPSResult::Lose => RPS::Rock,
                }
            },
            RPS::Scissors => {
                match result {
                    RPSResult::Win => RPS::Rock,
                    RPSResult::Draw => RPS::Scissors,
                    RPSResult::Lose => RPS::Paper,
                }
            },
        }
    }
}

#[derive(Clone, Copy)]
enum RPSResult {
    Win,
    Draw,
    Lose,
}

impl RPSResult {
    fn score(&self) -> u32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Lose => 0,
        }
    }
}

impl From<Symbol> for RPSResult {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::A | Symbol::B | Symbol::C => unimplemented!(),
            Symbol::X => Self::Lose,
            Symbol::Y => Self::Draw,
            Symbol::Z => Self::Win,
        }
    }
}

struct Round {
    opponent: Symbol,
    me: Symbol,
}

impl Round {
    fn result(&self) -> RPSResult {
        match self.me.into() {
            RPS::Rock => {
                match self.opponent.into() {
                    RPS::Rock => RPSResult::Draw,
                    RPS::Paper => RPSResult::Lose,
                    RPS::Scissors => RPSResult::Win,
                }
            },
            RPS::Paper => {
                match self.opponent.into() {
                    RPS::Rock => RPSResult::Win,
                    RPS::Paper => RPSResult::Draw,
                    RPS::Scissors => RPSResult::Lose,
                }
            },
            RPS::Scissors => {
                match self.opponent.into() {
                    RPS::Rock => RPSResult::Lose,
                    RPS::Paper => RPSResult::Win,
                    RPS::Scissors => RPSResult::Draw,
                }
            },
        }
    }

    fn score(&self) -> u32 {
        self.result().score() + RPS::from(self.me).score()
    }

    fn score2(&self) -> u32 {
        let opponent_hand = RPS::from(self.opponent);
        let result = RPSResult::from(self.me);
        let tuple = (opponent_hand, result);
        let my_hand = RPS::from(tuple);
        my_hand.score() + result.score()
    }
}

impl Parser for Round {
    fn parse_item<'a, I: Iterator<Item = String>>(line_iterator: &mut I) -> Result<Self, Box<dyn std::error::Error>> {
        let line = line_iterator.next();
        
        if let Some(line) = line {
            let split: Vec<&str> = line.split_whitespace().collect();
            let opponent = split[0].parse::<Symbol>()?;
            let me = split[1].parse::<Symbol>()?;

            return Ok(Round { opponent, me })
        }

        return Err(Box::new(ParseError::new("Empty line")));
    }
}

const TEST_STRING: &str = "A Y
B X
C Z";

const INPUT_FILE: &str = "./input/day2.txt";

#[test]
fn day2_part1_test() {
    let rounds = parse_input_string::<Round>(TEST_STRING.to_string()).unwrap();
    let result: u32 = rounds.iter().map(|round| round.score()).sum();
    let answer: u32 = 15;
    assert_eq!(result, answer);
}

pub fn day2_part1() -> u32 {
    let rounds = parse_input_file::<Round>(INPUT_FILE).unwrap();
    rounds.iter().map(|round| round.score()).sum()
}

#[test]
fn day2_part2_test() {
    let rounds = parse_input_string::<Round>(TEST_STRING.to_string()).unwrap();
    let result: u32 = rounds.iter().map(|round| round.score2()).sum();
    let answer: u32 = 12;
    assert_eq!(result, answer);
}

pub fn day2_part2() -> u32 {
    let rounds = parse_input_file::<Round>(INPUT_FILE).unwrap();
    rounds.iter().map(|round| round.score2()).sum()
}