#![warn(clippy::pedantic)]

use aoc2024::common::process_lines_from_args_file;
use aoc2024::solver::process_lines;

fn main() {
    let result = process_lines_from_args_file(process_lines);
    println!("The solution is `{result}`");
}
