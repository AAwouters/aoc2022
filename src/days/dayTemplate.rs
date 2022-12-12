use aoc_runner_derive::*;

type Input = u32;
type Output = u32;

#[aoc_generator(dayx)]
pub fn generator(input: &str) -> Vec<Input> {
    todo!()
}

#[aoc(dayx, part1)]
pub fn part1(input: &[Input]) -> Output {
    todo!()
}

#[aoc(dayx, part2)]
pub fn part2(input: &[Input]) -> Output {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let input = generator(&TEST_INPUT);
        assert_eq!(part2(&input), 45000);
    }
}
