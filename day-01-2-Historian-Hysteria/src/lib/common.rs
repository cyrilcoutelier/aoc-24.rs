use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Lines};
use std::iter::Map;
use std::path::Path;

pub trait ISolver {
    fn process_line(&self, line: &str);
    fn get_result(&self) -> String;
}

pub fn process_lines<I, S>(lines: I, solver: &S) -> String
where
    I: Iterator<Item = String>,
    S: ISolver,
{
    lines.for_each(|line| solver.process_line(&line));
    solver.get_result()
}

type FinalIterator = Map<Lines<BufReader<File>>, fn(Result<String, Error>) -> String>;

/// # Panics
///
/// Will panic if:
/// - No file path provided
/// - File not found
/// - Issue while reading the file
pub fn process_lines_from_args_file(process_lines_callback: fn(FinalIterator) -> String) -> String {
    let args: Vec<String> = env::args().collect();
    let path_str = args.get(1).expect("No file path provided");
    let path = Path::new(path_str);
    let file = File::open(path).unwrap_or_else(|_| panic!("file not found {path_str}"));
    let lines = io::BufReader::new(file).lines();
    let lines: FinalIterator = lines.map(Result::unwrap);

    process_lines_callback(lines)
}
