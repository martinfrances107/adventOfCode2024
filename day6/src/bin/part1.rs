use core::fmt::{write, Display, Formatter, Result};
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

// i32 as we walk to ask it there an obstacle as (-1, -1)
type Map = HashSet<(i32, i32)>;

// row, col
static DIRECTION: [[i32; 2]; 4] = [
    [-1, 0], // N
    [0, 1],  // E
    [1, 0],  // S
    [0, -1], // W
];
struct Turtle {
    row_max: usize,
    col_max: usize,
    row: usize,
    col: usize,
    // counts moves.
    moves: u32,
    direction_index: usize,
    map: Map,
    path: Path,
}

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
        writeln!(f, "")
    }
}

impl Path {
    fn squared_covered(&self) -> u32 {
        let mut covered = 0;
        for rows in &self.map {
            for c in rows.iter() {
                if *c == 'X' {
                    covered += 1;
                }
            }
        }
        covered
    }
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
        path.map[row][col] = 'X';
        Self {
            col_max,
            row_max,
            row,
            col,
            moves: 0,
            // Default to North
            direction_index: 0,
            map,
            path,
        }
    }

    fn advance(&mut self) -> bool {
        let mut next_row = self.row as i32 + DIRECTION[self.direction_index][0];
        let mut next_col = self.col as i32 + DIRECTION[self.direction_index][1];

        if self.map.contains(&(next_row, next_col)) {
            self.path.map[next_row as usize][next_col as usize] = '#';
            // rotate 90 before walking.
            if self.direction_index == 3 {
                self.direction_index = 0;
            } else {
                self.direction_index += 1;
            }
            next_row = self.row as i32 + DIRECTION[self.direction_index][0];
            next_col = self.col as i32 + DIRECTION[self.direction_index][1];
        }

        // Boundary checks.
        if next_row < 0
            || next_row > self.row_max as i32
            || next_col < 0
            || next_col > self.col_max as i32
        {
            true
        } else {
            // Advance.
            self.row = next_row as usize;
            self.col = next_col as usize;
            self.moves += 1;
            self.path.map[self.row][self.col] = 'X';

            false
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut col_max = 0;
    let mut row_max = 0;

    // contains only obstacles
    let mut map = Map::new();
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
                map.insert((row_index as i32, col_index as i32));
            }
        }
    }

    let mut turtle = Turtle::new(map, player_row, player_col, row_max, col_max);
    // advance until boundary
    'walk: loop {
        let crossed_boundary = turtle.advance();
        if crossed_boundary {
            break 'walk;
        }
    }

    turtle.path.squared_covered()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
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
        assert_eq!(part1(input), 41u32);
    }
}
