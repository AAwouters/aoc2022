mod util;
#[allow(dead_code)]
mod days;

use days::*;

fn main() {
    println!("Hello, Advent of Code!");

    let day1_result = day1_part1();
    let day1_part2_result = day1_part2();
    println!("day1: {}, part2: {}", day1_result, day1_part2_result);
}
