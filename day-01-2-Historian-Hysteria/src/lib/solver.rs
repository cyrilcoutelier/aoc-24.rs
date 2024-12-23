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

fn get_distance(pair: (i32, i32)) -> i32 {
    let (a, b) = pair;
    (a - b).abs()
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    let mut list: (Vec<i32>, Vec<i32>) = lines.map(|line| extract_numbers(&line)).unzip();
    list.0.sort_unstable();
    list.1.sort_unstable();

    let sum_of_distances: i32 = list.0.into_iter().zip(list.1).map(get_distance).sum();
    sum_of_distances.to_string()
}
