const MAX_DIFF: i32 = 3;
const MIN_DIFF: i32 = 1;

fn is_list_safe<F: Fn(i32, i32) -> bool>(list: &[i32], is_adjacent_acceptable: &F) -> bool {
    list.windows(2).all(|w| is_adjacent_acceptable(w[0], w[1]))
}

fn is_adjacent_diff_acceptable(left: i32, right: i32) -> bool {
    let diff = (left - right).abs();
    (MIN_DIFF..=MAX_DIFF).contains(&diff)
}

fn is_greater(left: i32, right: i32) -> bool {
    left > right
}

fn is_lower(left: i32, right: i32) -> bool {
    left < right
}

struct GreaterLower {
    nb_greater: usize,
    nb_lower: usize,
}

impl GreaterLower {
    fn new() -> Self {
        Self {
            nb_greater: 0,
            nb_lower: 0,
        }
    }
}

struct Report {
    list: Vec<i32>,
}

impl Report {
    pub fn new(list: Vec<i32>) -> Report {
        Report { list }
    }

    pub fn is_safe(&self) -> bool {
        let comparator = self.get_comparator();
        match comparator {
            Some(comparator) => self.test_adjacent(|left, right| {
                comparator(left, right) && is_adjacent_diff_acceptable(left, right)
            }),
            None => false,
        }
    }

    fn get_comparator(&self) -> Option<fn(i32, i32) -> bool> {
        let greater_lower = self.get_nb_greater_lower();
        match greater_lower.nb_greater.cmp(&greater_lower.nb_lower) {
            std::cmp::Ordering::Greater => Some(is_greater),
            std::cmp::Ordering::Less => Some(is_lower),
            std::cmp::Ordering::Equal => None,
        }
    }

    fn get_nb_greater_lower(&self) -> GreaterLower {
        self.list
            .windows(2)
            .fold(GreaterLower::new(), |mut acc, w| {
                match w[0].cmp(&w[1]) {
                    std::cmp::Ordering::Less => acc.nb_lower += 1,
                    std::cmp::Ordering::Greater => acc.nb_greater += 1,
                    std::cmp::Ordering::Equal => (),
                }
                acc
            })
    }

    fn test_adjacent<F: Fn(i32, i32) -> bool>(&self, callback: F) -> bool {
        is_list_safe(&self.list, &callback)
    }
}

/// # Panics
///
/// Will panic if:
/// - The string contains words without numbers
impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let list = value
            .split_whitespace()
            .map(|x| {
                x.parse::<i32>()
                    .unwrap_or_else(|_| panic!("The text {x} is not a number"))
            })
            .collect();
        Report::new(list)
    }
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    let reports = lines.map(|line| Report::from(line.as_str()));
    let nb_safe_reports = reports.filter(Report::is_safe).count();
    nb_safe_reports.to_string()
}
