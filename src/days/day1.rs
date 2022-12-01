use crate::util::{Parser, parse_input_string, parse_input_file};

type Calories = u32;

struct Elf {
    pub calories: Vec<Calories>,
}

impl Parser for Elf {
    fn parse_item<'a, I: Iterator<Item = String>>(line_iterator: &mut I) -> Result<Self, Box<dyn std::error::Error>> {
        let mut calories = Vec::new();

        loop {
            let line = line_iterator.next();

            if let Some(string) = line {
                if !string.is_empty() {
                    calories.push(string.parse::<Calories>()?);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(Elf {
            calories
        })
    }
}

const TEST_STRING: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

const INPUT_FILE: &str = "./input/day1.txt";

pub fn day1_part1_test() -> Calories {
    let elves = parse_input_string::<Elf>(TEST_STRING.to_string()).unwrap();
    elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).max().unwrap()
}

pub fn day1_part1() -> Calories {
    let elves = parse_input_file::<Elf>(INPUT_FILE).unwrap();
    elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).max().unwrap()
}

pub fn day1_part2_test() -> Calories {
    let elves = parse_input_string::<Elf>(TEST_STRING.to_string()).unwrap();
    let mut sums = elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).collect::<Vec<Calories>>();
    sums.sort_unstable();
    sums.reverse();
    let top3_sum = sums.iter().take(3).sum();
    top3_sum
}

pub fn day1_part2() -> Calories {
    let elves = parse_input_file::<Elf>(INPUT_FILE).unwrap();
    let mut sums = elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).collect::<Vec<Calories>>();
    sums.sort_unstable();
    sums.reverse();
    let top3_sum = sums.iter().take(3).sum();
    top3_sum
}
