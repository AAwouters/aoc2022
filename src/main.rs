mod util;
#[allow(dead_code)]
mod days;

use days::*;

fn main() {
    println!("Hello, Advent of Code!");

    // let day1_result = day1_part1();
    // let day1_part2_result = day1_part2();
    // print!("day1: {}", day1_result);
    // println!(", part2: {}", day1_part2_result);

    let day2_result = day2_part1();
    let day2_part2_result = day2_part2();
    print!("day2: {}", day2_result);
    println!(", part2: {}", day2_part2_result);
}
