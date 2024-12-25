#![warn(clippy::pedantic)]

use std::time::Instant;

use aoc2024::common::process_lines_from_args_file;
use aoc2024::solver::process_lines;

fn main() {
    let now = Instant::now();

    let result = process_lines_from_args_file(process_lines);
    println!("The solution is `{result}`");

    let elapsed = now.elapsed();
    println!("Solved in: {elapsed:.2?}");
}
