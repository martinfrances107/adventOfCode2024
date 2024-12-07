use core::fmt::{Display, Formatter, Result};
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

// i32 here.. As we walk is needs to ask is there an obstacle as (-1, -1)
type Map = HashSet<(i32, i32)>;

// row, col
static DIRECTION: [[i32; 2]; 4] = [
    [-1, 0], // N
    [0, 1],  // E
    [1, 0],  // S
    [0, -1], // W
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    direction_index: usize,
}

struct Turtle {
    row_max: usize,
    col_max: usize,
    state: State,
    moves: u32,
    first_pos: Option<State>,
    map: Map,
    path: Path,
}
#[derive(Clone)]
struct Path {
    map: Vec<Vec<char>>,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in &self.map {
            for c in row {
                write!(f, "{}", c)?
            }
            writeln!(f)?
        }
        writeln!(f)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    LoopDetected,
    Exit,
    Running,
}

impl Turtle {
    fn new(map: Map, row: usize, col: usize, row_max: usize, col_max: usize) -> Self {
        let mut path = Path { map: vec![] };
        for _row__index in 0..=row_max {
            let mut path_row = Vec::with_capacity(col_max);
            for _col_index in 0..=col_max {
                path_row.push('.');
            }
            path.map.push(path_row);
        }
        path.map[row][col] = '^';
        Self {
            col_max,
            row_max,
            state: State {
                row,
                col,
                // Default to North
                direction_index: 0,
            },
            first_pos: None,
            moves: 0,
            map,
            path,
        }
    }

    // Subtle: loop condition - because corners
    //
    // Must track the first position moved into not the origin point.
    //
    // This is because if the origin point is on a corner then coming from the right before moving
    // the direction changes mid walk and make tracking complex.
    fn advance(&mut self) -> Outcome {
        // println!("entry: Advance");
        let mut next_row = self.state.row as i32 + DIRECTION[self.state.direction_index][0];
        let mut next_col = self.state.col as i32 + DIRECTION[self.state.direction_index][1];
        let mut next_direction_index = self.state.direction_index;

        // Map "contains" a obstacle.
        if self.map.contains(&(next_row, next_col)) {
            self.path.map[next_row as usize][next_col as usize] = '#';
            // Rotate 90 before walking.
            next_direction_index = (self.state.direction_index + 1) % 4;
            next_row = self.state.row as i32 + DIRECTION[next_direction_index][0];
            next_col = self.state.col as i32 + DIRECTION[next_direction_index][1];
        }

        // Boundary checks.
        if next_row < 0
            || next_row > self.row_max as i32
            || next_col < 0
            || next_col > self.col_max as i32
        {
            Outcome::Exit
        } else {
            // Advance.
            self.state.row = next_row as usize;
            self.state.col = next_col as usize;
            self.state.direction_index = next_direction_index;
            self.moves = (self.moves + 1) % 10;

            self.path.map[self.state.row][self.state.col] = 'X';

            match &self.first_pos {
                Some(first_state) => {
                    if self.state == *first_state {
                        return Outcome::LoopDetected;
                    }
                }
                None => {
                    self.first_pos = Some(State {
                        row: self.state.row,
                        col: self.state.col,
                        direction_index: self.state.direction_index,
                    });
                }
            }

            Outcome::Running
        }
    }

    fn walk(&mut self) -> Outcome {
        let mut attempts = 0;
        // Advance until boundary or loop detected.
        loop {
            match self.advance() {
                Outcome::Exit => return Outcome::Exit,
                Outcome::LoopDetected => return Outcome::LoopDetected,
                Outcome::Running => {
                    if attempts > 9999 {
                        return Outcome::LoopDetected;
                    }
                }
            }
            attempts += 1;
        }
    }
}

fn part2(input: &str) -> u32 {
    let mut col_max = 0;
    let mut row_max = 0;

    // Contains only obstacles.
    let mut original_map = Map::new();
    let mut player_col = 0;
    let mut player_row = 0;

    // Construct map and turtle.
    for (row_index, row) in input.lines().enumerate() {
        row_max = row_index;
        for (col_index, c) in row.chars().enumerate() {
            if col_index > col_max {
                col_max = col_index;
            }

            if c == '^' {
                // Player position.
                player_row = row_index;
                player_col = col_index;
            }

            // Add obstacle to map.
            if c == '#' {
                original_map.insert((row_index as i32, col_index as i32));
            }
        }
    }

    let mut loop_count = 0;
    let mut turtle = Turtle::new(original_map, player_row, player_col, row_max, col_max);
    for (row_index, row) in input.lines().enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            if c != '#' {
                // let map = original_map.clone();
                // debug_assert_eq!(original_map.len(), map.len());
                // let mut turtle = Turtle::new(map, player_row, player_col, row_max, col_max);
                let obs_inserted = turtle.map.insert((row_index as i32, col_index as i32));
                debug_assert!(obs_inserted);

                match turtle.walk() {
                    Outcome::LoopDetected => {
                        loop_count += 1;
                        turtle.state = State {
                            row: player_row,
                            col: player_col,
                            direction_index: 0,
                        };
                        turtle.first_pos = None;
                        turtle.moves = 0;
                        // let success = turtle.map.remove(&(row_index as i32, col_index as i32));
                        // debug_assert!(success);
                    }
                    Outcome::Exit => {
                        // Obstacle in this position did not lead to a loop.
                        turtle.state = State {
                            row: player_row,
                            col: player_col,
                            direction_index: 0,
                        };
                        turtle.first_pos = None;
                        turtle.moves = 0;
                    }

                    Outcome::Running => {
                        panic!("Cannot stop walking while running");
                    }
                }

                let success = turtle.map.remove(&(row_index as i32, col_index as i32));
                debug_assert!(success);
            }
        }
    }

    loop_count
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn no_obstacle() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let mut col_max = 0;
        let mut row_max = 0;

        // contains only obstacles
        let mut original_map = Map::new();
        let mut player_col = 0;
        let mut player_row = 0;

        // Construct map and turtle
        for (row_index, row) in input.lines().enumerate() {
            row_max = row_index;
            for (col_index, c) in row.chars().enumerate() {
                if col_index > col_max {
                    col_max = col_index;
                }

                if c == '^' {
                    // player position.
                    player_row = row_index;
                    player_col = col_index;
                }

                // Add obstacle to map
                if c == '#' {
                    original_map.insert((row_index as i32, col_index as i32));
                }
            }
        }

        let mut turtle = Turtle::new(original_map, player_row, player_col, row_max, col_max);
        println!("{}", turtle.path);
        assert_eq!(turtle.walk(), Outcome::Exit);
    }
    #[test]
    fn printing_press() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#...";

        let mut col_max = 0;
        let mut row_max = 0;

        // contains only obstacles
        let mut original_map = Map::new();
        let mut player_col = 0;
        let mut player_row = 0;

        // Construct map and turtle
        for (row_index, row) in input.lines().enumerate() {
            row_max = row_index;
            for (col_index, c) in row.chars().enumerate() {
                if col_index > col_max {
                    col_max = col_index;
                }

                if c == '^' {
                    // Player position.
                    player_row = row_index;
                    player_col = col_index;
                }

                // Add obstacle to map.
                if c == '#' {
                    original_map.insert((row_index as i32, col_index as i32));
                }
            }
        }

        let mut turtle = Turtle::new(original_map, player_row, player_col, row_max, col_max);

        assert_eq!(turtle.walk(), Outcome::LoopDetected);
    }

    #[test]
    fn suit_prototypes() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
......#.#.
#.........
......#...";

        let mut col_max = 0;
        let mut row_max = 0;

        // contains only obstacles
        let mut original_map = Map::new();
        let mut player_col = 0;
        let mut player_row = 0;

        // Construct map and turtle
        for (row_index, row) in input.lines().enumerate() {
            row_max = row_index;
            for (col_index, c) in row.chars().enumerate() {
                if col_index > col_max {
                    col_max = col_index;
                }

                if c == '^' {
                    // player position.
                    player_row = row_index;
                    player_col = col_index;
                }

                // Add obstacle to map
                if c == '#' {
                    original_map.insert((row_index as i32, col_index as i32));
                }
            }
        }

        let mut turtle = Turtle::new(original_map, player_row, player_col, row_max, col_max);
        println!("{}", turtle.path);
        assert_eq!(turtle.walk(), Outcome::LoopDetected);
    }

    #[test]
    fn chimney_squeze() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
.......##.
#.........
......#...";

        let mut col_max = 0;
        let mut row_max = 0;

        // contains only obstacles
        let mut original_map = Map::new();
        let mut player_col = 0;
        let mut player_row = 0;

        // Construct map and turtle
        for (row_index, row) in input.lines().enumerate() {
            row_max = row_index;
            for (col_index, c) in row.chars().enumerate() {
                if col_index > col_max {
                    col_max = col_index;
                }

                if c == '^' {
                    // Player position.
                    player_row = row_index;
                    player_col = col_index;
                }

                // Add obstacle to map.
                if c == '#' {
                    original_map.insert((row_index as i32, col_index as i32));
                }
            }
        }

        let mut turtle = Turtle::new(original_map, player_row, player_col, row_max, col_max);
        println!("{}", turtle.path);
        assert_eq!(turtle.walk(), Outcome::LoopDetected);
    }

    #[test]
    fn example2() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part2(input), 6u32);
    }
}
