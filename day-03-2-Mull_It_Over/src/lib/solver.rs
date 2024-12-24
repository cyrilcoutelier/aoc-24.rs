use std::cmp::Ordering;
use std::{cell::Cell, collections::BTreeSet};

use crate::common::{self, ISolver};

const MAX_DIGIT: usize = 3;
const MUL_HEADER: &str = "mul";
const MUL_HEADER_LEN: usize = MUL_HEADER.len();
const DO_HEADER: &str = "do()";
const DO_NO_HEADER: &str = "don't()";

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    index: usize,
    payload: Payload,
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

#[derive(Eq, PartialEq, Debug)]
enum Payload {
    Do,
    DoNot,
    Mul(i32, i32),
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

    Some(Instruction {
        index,
        payload: Payload::Mul(left, right),
    })
}

struct Parser {
    is_enabled: bool,
    result: i32,
    instructions: Cell<BTreeSet<Instruction>>,
}

impl Parser {
    fn new() -> Self {
        Self {
            is_enabled: true,
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

    fn parse_do_instructions(&mut self, input: &str) {
        input.match_indices(DO_HEADER).for_each(|(idx, _)| {
            self.instructions.get_mut().insert(Instruction {
                index: idx,
                payload: Payload::Do,
            });
        });
    }

    fn parse_dont_instructions(&mut self, input: &str) {
        input.match_indices(DO_NO_HEADER).for_each(|(idx, _)| {
            self.instructions.get_mut().insert(Instruction {
                index: idx,
                payload: Payload::DoNot,
            });
        });
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction.payload {
            Payload::Do => self.is_enabled = true,
            Payload::DoNot => self.is_enabled = false,
            Payload::Mul(left, right) => {
                if self.is_enabled {
                    self.result += left * right;
                }
            }
        }
    }
}

impl ISolver for Parser {
    fn process_line(&mut self, line: &str) {
        self.parse_mul_instructions(line);
        self.parse_do_instructions(line);
        self.parse_dont_instructions(line);
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
