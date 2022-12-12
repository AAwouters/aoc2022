use aoc_runner_derive::*;

type Pair = (u32, u32);

type Input = (Pair, Pair);
type Output = u32;

fn full_overlap(input: &Input) -> bool {
    let left = input.0;
    let right = input.1;

    left.0 <= right.0 && left.1 >= right.1 || left.0 >= right.0 && left.1 <= right.1
}

fn partial_overlap(input: &Input) -> bool {
    if full_overlap(input) {
        return true;
    }

    let left = input.0;
    let right = input.1;

    right.0 <= left.0 && left.0 <= right.1
        || right.0 <= left.1 && left.1 <= right.1
        || left.0 <= right.0 && right.0 <= left.1
        || left.0 <= right.1 && right.1 <= left.1
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let mut values = line.split([',', '-']);
            (
                (
                    values.next().unwrap().parse().unwrap(),
                    values.next().unwrap().parse().unwrap(),
                ),
                (
                    values.next().unwrap().parse().unwrap(),
                    values.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Input]) -> Output {
    input
        .iter()
        .map(|input| if full_overlap(input) { 1 } else { 0 })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Input]) -> Output {
    input
        .iter()
        .map(|input| if partial_overlap(input) { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part2(&input), 4);
    }
}
