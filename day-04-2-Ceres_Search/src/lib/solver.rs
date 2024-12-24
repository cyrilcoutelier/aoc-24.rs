use crate::common::{self, ISolver};

const FIRST_CHAR: char = 'A';
const CHARS_SEQUENCE: [char; 4] = ['M', 'M', 'S', 'S'];

type Coordinate = (i32, i32);

const POSITIONS_SEQUENCE: [Coordinate; 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

fn add_coordinates(a: Coordinate, b: Coordinate) -> Coordinate {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 + x2, y1 + y2)
}

struct Solver {
    tab: Vec<Vec<char>>,
}

impl Solver {
    pub fn new() -> Self {
        Self { tab: vec![] }
    }

    fn get_all_matches(&self) -> usize {
        let width = i32::try_from(self.get_width()).unwrap();
        let height = i32::try_from(self.get_height()).unwrap();
        let mut matches = 0;
        for y in 0..height {
            for x in 0..width {
                if self.test_position((x, y)) {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn test_position(&self, position: Coordinate) -> bool {
        let Some(letter) = self.get_letter(position) else {
            return false;
        };
        if letter != FIRST_CHAR {
            return false;
        }

        (0..CHARS_SEQUENCE.len()).any(|index| self.test_combination(position, index))
    }

    fn test_combination(&self, origin: Coordinate, shift: usize) -> bool {
        POSITIONS_SEQUENCE
            .iter()
            .enumerate()
            .all(|(index, vector)| {
                let letter_index = (index + shift) % CHARS_SEQUENCE.len();
                let searched_letter = CHARS_SEQUENCE[letter_index];
                let position = add_coordinates(origin, *vector);
                let Some(letter) = self.get_letter(position) else {
                    return false;
                };
                letter == searched_letter
            })
    }

    fn get_letter(&self, position: Coordinate) -> Option<char> {
        let (x, y) = position;
        if !self.is_coordinate_inside(position) {
            return None;
        }
        let x = usize::try_from(x).unwrap();
        let y = usize::try_from(y).unwrap();
        Some(self.tab[y][x])
    }

    fn is_coordinate_inside(&self, point: Coordinate) -> bool {
        let (x, y) = point;
        let width = i32::try_from(self.get_width()).unwrap();
        let height = i32::try_from(self.get_height()).unwrap();
        y >= 0 && y < height && x >= 0 && x < width
    }

    fn get_width(&self) -> usize {
        self.tab[0].len()
    }

    fn get_height(&self) -> usize {
        self.tab.len()
    }
}

impl ISolver for Solver {
    fn process_line(&mut self, line: &str) {
        self.tab.push(line.chars().collect());
    }

    fn get_result(&mut self) -> String {
        self.get_all_matches().to_string()
    }
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    common::process_lines(lines, Solver::new())
}
