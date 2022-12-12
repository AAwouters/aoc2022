use aoc_runner_derive::*;

type Input = String;
type Output = u32;

fn find_common(contents: &str) -> char {
    let split = contents.split_at(contents.len() / 2);
    for char in split.0.chars() {
        if split.1.contains(char) {
            return char;
        }
    }
    unreachable!()
}

fn find_tripe_common(line1: &String, line2: &String, line3: &String) -> char {
    for char in line1.chars() {
        if line2.contains(char) && line3.contains(char) {
            return char;
        }
    }
    unreachable!()
}

fn priority(item: &char) -> u32 {
    if item.is_lowercase() {
        return (*item as u32) - 96;
    } else if item.is_uppercase() {
        return (*item as u32) - 38;
    }
    unreachable!()
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Input> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Input]) -> Output {
    input
        .iter()
        .map(|line| {
            let common = find_common(line);
            priority(&common)
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Input]) -> Output {
    let mut iterator = input.iter().peekable();
    let mut sum = 0;

    while iterator.peek().is_some() {
        let line1 = iterator.next().unwrap();
        let line2 = iterator.next().unwrap();
        let line3 = iterator.next().unwrap();

        let char = find_tripe_common(line1, line2, line3);
        sum += priority(&char);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part2(&input), 70);
    }
}
