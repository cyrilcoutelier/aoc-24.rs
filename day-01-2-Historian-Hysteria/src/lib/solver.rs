use std::collections::HashMap;

/// # Panics
///
/// Will panic if:
/// - The string does not contain two numbers separated by spaces
pub fn extract_numbers(line: &str) -> (i32, i32) {
    let mut words = line.split(' ');
    let first_number: i32 = words.next().unwrap().parse().unwrap();
    let last_number: i32 = words.last().unwrap().parse().unwrap();
    (first_number, last_number)
}

fn list_to_occurrences(list: Vec<i32>) -> HashMap<i32, i32> {
    let mut occurrences = HashMap::new();
    for number in list {
        let count = occurrences.entry(number).or_insert(0);
        *count += 1;
    }
    occurrences
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    let (left_list, right_list): (Vec<i32>, Vec<i32>) =
        lines.map(|line| extract_numbers(&line)).unzip();
    let right_list = list_to_occurrences(right_list);
    let sum_of_similarity_scores: i32 = left_list
        .into_iter()
        .map(|number| {
            let count = right_list.get(&number).unwrap_or(&0);
            number * count
        })
        .sum();
    sum_of_similarity_scores.to_string()
}
