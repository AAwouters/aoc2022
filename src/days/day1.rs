use aoc_runner_derive::*;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calory_str| calory_str.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut calory_sums: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    calory_sums.sort_unstable();
    calory_sums.reverse();
    *calory_sums.iter().take(1).next().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut calory_sums: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    calory_sums.sort_unstable();
    calory_sums.reverse();
    calory_sums.iter().take(3).sum()
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
