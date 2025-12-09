use std::{fs, time::Instant};

mod solution;

fn main() {
    let input = load_input("./input.txt");

    let start = Instant::now();
    let part1_result = solution::part1(&input);
    let t1 = start.elapsed();

    println!("part 1 finished in {:?}", t1);
    println!("The result for part 1 is: {}", part1_result);

    let start = Instant::now();
    let part2_result = solution::part2(&input);
    let t2 = start.elapsed();

    println!("part 2 finished in {:?}", t2);
    println!("The result for part 2 is: {}", part2_result);
    println!("Total runtime was {:?}", t1 + t2);
}

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}
