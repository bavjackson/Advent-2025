use std::{fs, time::Instant};

use crate::solution::parse_input;

mod solution;

fn main() {
    let input = load_input("./input.txt");

    let start = Instant::now();
    let (mut ranges, ids) = parse_input(&input);
    let t_input = start.elapsed();

    println!("input parsing finished in {:?}", t_input);

    let start = Instant::now();
    let part1_result = solution::part1(&ranges, ids);
    let t1 = start.elapsed();

    println!("part 1 finished in {:?}", t1);
    println!("The result for part 1 is: {}", part1_result);

    let start = Instant::now();
    let part2_result = solution::part2(&mut ranges);
    let t2 = start.elapsed();

    println!("part 2 finished in {:?}", t2);
    println!("The result for part 2 is: {}", part2_result);
    println!("Total runtime was {:?}", t_input + t1 + t2);
}

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}
