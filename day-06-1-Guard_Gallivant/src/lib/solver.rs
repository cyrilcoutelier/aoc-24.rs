use std::{collections::HashSet, hash::Hash};

#[derive(Copy, Clone)]
enum Tile {
    Wall,
    Open,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn add_coordinates(a: Coordinate, b: Coordinate) -> Coordinate {
    Coordinate {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

const DIRECTIONS_LIST: [Coordinate; 4] = [
    Coordinate { x: 0, y: -1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 0, y: 1 },
    Coordinate { x: -1, y: 0 },
];

struct MapParser {
    map: Vec<Vec<Tile>>,
    origin: Option<Coordinate>,
}

impl MapParser {
    fn new() -> Self {
        MapParser {
            map: Vec::new(),
            origin: None,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let row = line
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                '^' => {
                    let y = i32::try_from(self.map.len()).unwrap();
                    let x = i32::try_from(x).unwrap();
                    self.origin = Some(Coordinate { x, y });
                    Tile::Open
                }
                _ => panic!("Invalid character in map {c}"),
            })
            .collect();
        self.map.push(row);
    }

    fn get_map_data(self) -> MapData {
        MapData {
            map: self.map,
            origin: self.origin.expect("No origin found in data"),
        }
    }
}

struct MapData {
    map: Vec<Vec<Tile>>,
    origin: Coordinate,
}

struct MapSolver {
    map: Vec<Vec<Tile>>,
    guard: Guard,
    visited_set: HashSet<Coordinate>,
}

impl MapSolver {
    fn new(map_data: MapData) -> Self {
        Self {
            map: map_data.map,
            guard: Guard::new(map_data.origin),
            visited_set: HashSet::new(),
        }
    }

    fn solve(&mut self) -> String {
        self.walk_map();
        self.visited_set.len().to_string()
    }

    fn walk_map(&mut self) {
        while self.is_guard_in_map() {
            self.update_visited();
            self.update_position();
        }
    }

    fn is_guard_in_map(&self) -> bool {
        self.is_position_in_map(self.guard.position)
    }

    fn is_position_in_map(&self, position: Coordinate) -> bool {
        let width = i32::try_from(self.map[0].len()).unwrap();
        let height = i32::try_from(self.map.len()).unwrap();

        position.x >= 0 && position.x < width && position.y >= 0 && position.y < height
    }

    fn update_visited(&mut self) {
        self.visited_set.insert(self.guard.position);
    }

    fn update_position(&mut self) {
        if self.is_guard_facing_wall() {
            self.guard.turn();
        } else {
            self.guard.move_forward();
        }
    }

    fn is_guard_facing_wall(&self) -> bool {
        let facing_position = self.guard.get_facing_position();
        matches!(self.get_tile(facing_position), Some(Tile::Wall))
    }

    fn get_tile(&self, position: Coordinate) -> Option<Tile> {
        let Ok(x) = usize::try_from(position.x) else {
            // x is below 0
            return None;
        };
        let Ok(y) = usize::try_from(position.y) else {
            // y is below 0
            return None;
        };
        let row = self.map.get(y)?;
        row.get(x).copied()
    }
}

struct Guard {
    position: Coordinate,
    direction_index: usize,
}

impl Guard {
    fn new(position: Coordinate) -> Self {
        Self {
            position,
            direction_index: 0,
        }
    }

    fn turn(&mut self) {
        self.direction_index = (self.direction_index + 1) % DIRECTIONS_LIST.len();
    }

    fn move_forward(&mut self) {
        self.position = self.get_facing_position();
    }

    fn get_facing_position(&self) -> Coordinate {
        let direction = self.get_direction();
        add_coordinates(self.position, direction)
    }

    fn get_direction(&self) -> Coordinate {
        DIRECTIONS_LIST[self.direction_index]
    }
}

pub fn process_lines<T>(lines: T) -> String
where
    T: Iterator<Item = String>,
{
    let mut parser = MapParser::new();
    for line in lines {
        parser.parse_line(&line);
    }
    let map_data = parser.get_map_data();

    let mut map_solver = MapSolver::new(map_data);
    map_solver.solve()
}
