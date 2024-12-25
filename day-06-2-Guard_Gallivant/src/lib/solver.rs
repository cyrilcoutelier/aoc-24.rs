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

struct MapWalker<'a> {
    map: &'a [Vec<Tile>],
    initial_position: Coordinate,
    guard: Guard,
    visited_set: HashSet<Coordinate>,
    added_obstruction: Option<Coordinate>,
    states_history: HashSet<Guard>,
}

impl<'a> MapWalker<'a> {
    fn new(
        map: &'a [Vec<Tile>],
        initial_position: Coordinate,
        added_obstruction: Option<Coordinate>,
    ) -> Self {
        Self {
            map,
            initial_position,
            guard: Guard::new(initial_position),
            visited_set: HashSet::new(),
            added_obstruction,
            states_history: HashSet::new(),
        }
    }

    /// Takes slightly less than a second compiled on release mode on my M1
    /// Possible optimization would be to use rayon to parallelize the search
    fn solve(&mut self) -> String {
        self.walk_map();

        let candidate_list = self.get_visited_position_without_initial();
        let nb_possible_obstructions = candidate_list
            .iter()
            .filter(|&x| {
                self.added_obstruction = Some(*x);
                self.walk_map()
            })
            .count();

        nb_possible_obstructions.to_string()
    }

    /// Returns true if stuck in a loop, false if went outside of the map
    fn walk_map(&mut self) -> bool {
        self.reset_guard();

        while self.is_guard_in_map() {
            if self.added_obstruction.is_none() {
                self.update_visited();
            } else {
                if self.states_history.contains(&self.guard) {
                    return true;
                }
                self.states_history.insert(self.guard);
            }

            self.update_position();
        }
        false
    }

    fn reset_guard(&mut self) {
        self.guard = Guard::new(self.initial_position);
        self.states_history.clear();
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
        if let Some(obstruction) = self.added_obstruction {
            if obstruction == position {
                return Some(Tile::Wall);
            }
        }

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

    fn get_visited_position_without_initial(&self) -> Vec<Coordinate> {
        self.visited_set
            .iter()
            .filter(|&x| *x != self.initial_position)
            .copied()
            .collect()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    let MapData { map, origin } = parser.get_map_data();

    let mut map_solver = MapWalker::new(&map, origin, None);
    map_solver.solve()
}
