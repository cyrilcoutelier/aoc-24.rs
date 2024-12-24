const MAX_DIGIT: usize = 3;

struct Instruction {
    left: i32,
    right: i32,
}

impl Instruction {
    fn solve(&self) -> i32 {
        self.left * self.right
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(mut input: &str) -> Result<Self, Self::Error> {
        let parenthesis_left_idx = input.find('(').ok_or(())?;
        if parenthesis_left_idx > 0 {
            return Err(());
        }
        input = &input[1..];

        let comma_idx = input.find(',').ok_or(())?;
        let left_number = &input[..comma_idx];
        if left_number.is_empty()
            || left_number.len() > MAX_DIGIT
            || left_number.chars().any(|c| !c.is_ascii_digit())
        {
            return Err(());
        }
        let left = left_number.parse().unwrap();

        input = &input[comma_idx + 1..];

        let right_parenthesis_idx = input.find(')').ok_or(())?;
        let right_number = &input[..right_parenthesis_idx];
        if right_number.is_empty()
            || right_number.len() > MAX_DIGIT
            || right_number.chars().any(|c| !c.is_ascii_digit())
        {
            return Err(());
        }
        let right = right_number.parse().unwrap();

        Ok(Instruction { left, right })
    }
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    lines
        .map(|line| {
            let list = line.split("mul");
            list.skip(1)
                .flat_map(Instruction::try_from)
                .map(|x| x.solve())
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}
