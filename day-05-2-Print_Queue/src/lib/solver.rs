use std::collections::BTreeMap;

use crate::common::{self, ISolver};

fn get_middle_number(update: &[i32]) -> i32 {
    let len = update.len();
    let mid_index = len / 2;
    update[mid_index]
}

struct Solver {
    previous_map: BTreeMap<i32, Vec<i32>>,
    result: i32,
}

impl Solver {
    fn new() -> Self {
        Self {
            previous_map: BTreeMap::new(),
            result: 0,
        }
    }

    fn process_ordering(&mut self, line: &str) {
        let mut numbers = line.split('|').map(|x| x.parse::<i32>().unwrap());
        let first = numbers.next().unwrap();
        let second = numbers.next().unwrap();

        let vec = self.previous_map.entry(second).or_default();
        vec.push(first);
    }

    fn process_update(&mut self, line: &str) {
        let update: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        if self.is_correctly_ordered(&update) {
            self.result += get_middle_number(&update);
        }
    }

    fn is_correctly_ordered(&self, update: &[i32]) -> bool {
        (0..update.len()).all(|i| {
            let tested_entry = update[i];
            if let Some(previous_list) = self.previous_map.get(&tested_entry) {
                let next_entries = &update[i + 1..];
                previous_list
                    .iter()
                    .all(|previous_item| !next_entries.contains(previous_item))
            } else {
                true
            }
        })
    }
}

impl ISolver for Solver {
    fn process_line(&mut self, line: &str) {
        if line.contains('|') {
            self.process_ordering(line);
        } else if line.contains(',') {
            self.process_update(line);
        }
    }

    fn get_result(&mut self) -> String {
        self.result.to_string()
    }
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    common::process_lines(lines, Solver::new())
}
