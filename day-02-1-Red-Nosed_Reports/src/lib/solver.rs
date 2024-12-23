const MAX_DIFF: i32 = 3;
const MIN_DIFF: i32 = 1;

struct Report {
    list: Vec<i32>,
}

impl Report {
    pub fn new(list: Vec<i32>) -> Report {
        Report { list }
    }

    pub fn is_safe(&self) -> bool {
        self.are_valid_diff() && (self.is_increasing() || self.is_decreasing())
    }

    fn are_valid_diff(&self) -> bool {
        self.list.windows(2).all(|w| {
            let diff = (w[0] - w[1]).abs();
            (MIN_DIFF..=MAX_DIFF).contains(&diff)
        })
    }

    fn is_increasing(&self) -> bool {
        self.list.windows(2).all(|w| w[0] < w[1])
    }

    fn is_decreasing(&self) -> bool {
        self.list.windows(2).all(|w| w[0] > w[1])
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
