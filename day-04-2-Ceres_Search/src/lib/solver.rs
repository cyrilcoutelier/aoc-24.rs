use crate::common::{self, ISolver};

const SEARCHED_WORD: [char; 4] = ['X', 'M', 'A', 'S'];

type Coordinate = (i32, i32);

const VECTORS: [Coordinate; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

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
                matches += self.get_matches_at_position((x, y));
            }
        }
        matches
    }

    fn get_matches_at_position(&self, position: Coordinate) -> usize {
        VECTORS
            .iter()
            .filter(|&vector| self.test_word(position, *vector))
            .count()
    }

    fn test_word(&self, mut position: Coordinate, vector: Coordinate) -> bool {
        for &searched_letter in &SEARCHED_WORD {
            let Some(letter) = self.get_letter(position) else {
                return false;
            };
            if letter != searched_letter {
                return false;
            }
            position = add_coordinates(position, vector);
        }
        true
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
