use std::cmp::Ordering;
use std::{cell::Cell, collections::BTreeSet};

use crate::common::{self, ISolver};

const MAX_DIGIT: usize = 3;
const MUL_HEADER: &str = "mul";
const MUL_HEADER_LEN: usize = MUL_HEADER.len();

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    index: usize,
    left: i32,
    right: i32,
}

impl Ord for Instruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Instruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

fn try_parse_mul(mut input: &str, index: usize) -> Option<Instruction> {
    let parenthesis_left_idx = input.find('(')?;
    if parenthesis_left_idx != MUL_HEADER_LEN {
        return None;
    }
    input = &input[(MUL_HEADER_LEN + 1)..];

    let comma_idx = input.find(',')?;
    let left_number = &input[..comma_idx];
    if left_number.is_empty()
        || left_number.len() > MAX_DIGIT
        || left_number.chars().any(|c| !c.is_ascii_digit())
    {
        return None;
    }
    let left = left_number.parse().unwrap();

    input = &input[comma_idx + 1..];

    let right_parenthesis_idx = input.find(')')?;
    let right_number = &input[..right_parenthesis_idx];
    if right_number.is_empty()
        || right_number.len() > MAX_DIGIT
        || right_number.chars().any(|c| !c.is_ascii_digit())
    {
        return None;
    }
    let right = right_number.parse().unwrap();

    Some(Instruction { index, left, right })
}

struct Parser {
    result: i32,
    instructions: Cell<BTreeSet<Instruction>>,
}

impl Parser {
    fn new() -> Self {
        Self {
            result: 0,
            instructions: Cell::new(BTreeSet::new()),
        }
    }

    fn parse_mul_instructions(&mut self, input: &str) {
        input
            .match_indices(MUL_HEADER)
            .filter_map(|(idx, _)| {
                let instruction_input = &input[idx..];
                try_parse_mul(instruction_input, idx)
            })
            .for_each(|instruction| {
                self.instructions.get_mut().insert(instruction);
            });
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        self.result += instruction.left * instruction.right;
    }
}

impl ISolver for Parser {
    fn process_line(&mut self, line: &str) {
        self.parse_mul_instructions(line);
        let instructions = self.instructions.replace(BTreeSet::new());
        for instruction in instructions {
            self.process_instruction(&instruction);
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
    common::process_lines(lines, Parser::new())
}
