use crate::util::util_imports::*;

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

#[test]
fn day1_part1_test() {
    let elves = parse_input_string::<Elf>(TEST_STRING.to_string()).unwrap();
    let result = elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).max().unwrap();
    let answer: u32 = 24000;
    assert_eq!(result, answer);
}

pub fn day1_part1() -> Calories {
    let elves = parse_input_file::<Elf>(INPUT_FILE).unwrap();
    elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).max().unwrap()
}

#[test]
fn day1_part2_test() {
    let elves = parse_input_string::<Elf>(TEST_STRING.to_string()).unwrap();
    let mut sums = elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).collect::<Vec<Calories>>();
    sums.sort_unstable();
    sums.reverse();
    let top3_sum: u32 = sums.iter().take(3).sum();
    let answer: u32 = 45000;
    assert_eq!(top3_sum, answer);
}

pub fn day1_part2() -> Calories {
    let elves = parse_input_file::<Elf>(INPUT_FILE).unwrap();
    let mut sums = elves.iter().map(|elf| elf.calories.iter().sum::<Calories>()).collect::<Vec<Calories>>();
    sums.sort_unstable();
    sums.reverse();
    let top3_sum = sums.iter().take(3).sum();
    top3_sum
}
