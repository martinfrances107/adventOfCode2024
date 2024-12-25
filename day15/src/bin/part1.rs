use core::fmt::Display;
use core::panic;
use std::collections::HashMap;

use glam::IVec2;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::IResult;

fn main() {
    // let input = include_str!("./input1.txt");
    let map_str = include_str!("./input1.txt");
    let dir_str = include_str!("./dirs.txt");
    println!("{:?}", part1(map_str, dir_str));
}

// adjacent, ahead

#[derive(Debug, Clone)]
enum Object {
    Box,
    Blank,
    Player,
    Wall,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Blank => {
                write!(f, ".")?;
            }
            Object::Wall => {
                write!(f, "#")?;
            }
            Object::Player => {
                write!(f, "@")?;
            }
            Object::Box => {
                write!(f, "O")?;
            }
        }
        Ok(())
    }
}
fn parse_map_row(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_map(input: &str) -> IResult<&str, HashMap<(i32, i32), Object>> {
    let mut map = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let object = match c {
                '#' => Object::Wall,
                'O' => Object::Box,
                '.' => Object::Blank,
                _ => panic!(),
            };
            map.insert((row as i32, col as i32), object);
        }
    }

    Ok(("", map))
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::N => write!(f, "N")?,
            Direction::E => write!(f, "E")?,
            Direction::S => write!(f, "S")?,
            Direction::W => write!(f, "W")?,
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Warehouse {
    n_rows: usize,
    n_cols: usize,
    map: Vec<Vec<Object>>,
    player_pos: IVec2,
    direction: HashMap<Direction, (IVec2, IVec2)>,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        // adjacent space, space head
        let mut direction = HashMap::new();
        direction.insert(Direction::N, (IVec2::new(-1, 0), IVec2::new(-2, 0)));
        direction.insert(Direction::E, (IVec2::new(0, 1), IVec2::new(0, 2)));
        direction.insert(Direction::S, (IVec2::new(1, 0), IVec2::new(2, 0)));
        direction.insert(Direction::W, (IVec2::new(0, -1), IVec2::new(0, -2)));

        let mut player_pos = IVec2::new(0, 0);
        let mut map = vec![];

        let mut n_rows = 0;
        let mut n_cols = 0;
        for (row, line) in input.lines().enumerate() {
            n_rows += 1;
            n_cols = 0;
            map[row] = vec![];
            for (col, c) in line.chars().enumerate() {
                n_cols += 1;
                let o = match c {
                    '#' => Object::Wall,
                    'O' => Object::Box,
                    '@' => {
                        player_pos = (row as i32, col as i32).into();
                        Object::Player
                    }
                    '.' => Object::Blank,
                    _ => panic!("bad decode"),
                };
                map[row].push(o);
            }
        }
        dbg!(n_cols);
        dbg!(n_rows);
        Self {
            n_rows,
            n_cols,
            map,
            player_pos,
            direction,
        }
    }
    fn shift(&mut self, dir: &Direction) {
        let (adjacent_offset, ahead_offset) = self
            .direction
            .get(&dir)
            .expect("must have a adjacent and ahead");
        // dbg!(adjacent_offset);
        // dbg!(ahead_offset);

        let adjacent_pos = self.player_pos + adjacent_offset;
        let adjacent_object = self.map.get(&adjacent_pos).expect("Must have a adjacent");
        println!("adjacent {} {}", adjacent_pos, adjacent_object);

        match adjacent_object {
            Object::Blank => {
                // Move player into blank space
                println!("moving into blank space");
                self.map.insert(adjacent_pos, Object::Player);
                self.map.insert(self.player_pos, Object::Blank);
                self.player_pos = adjacent_pos;
            }
            Object::Box => {
                // Look at space ahead
                let ahead_pos = self.player_pos + ahead_offset;
                println!("adjacent block is O");
                match self.map.get(&ahead_pos) {
                    Some(Object::Blank) => {
                        println!("Shifting the block ahead '.' {}", ahead_pos);
                        // shift object.
                        self.map.insert(ahead_pos, Object::Box);
                        self.map.insert(adjacent_pos, Object::Player);
                        self.map.insert(self.player_pos, Object::Blank);
                        self.player_pos = adjacent_pos;
                    }
                    Some(Object::Player) => panic!("Should not find another player"),
                    Some(Object::Wall) | Some(Object::Box) | None => {
                        println!("No movement");
                        // Ahead was a wall, another object or off the map.
                    }
                };
            }
            Object::Player | Object::Wall => {
                // No move possible
            }
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(self.n_cols);
        dbg!(self.n_cols);
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                let key = IVec2::new(row as i32, col as i32);
                let object = self.map.get(&key).expect("must decode ");
                write!(f, "{}", *object)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
fn part1(map_str: &str, dirs: &str) -> u32 {
    let mut w = Warehouse::new(map_str);
    let d = dirs
        .lines()
        .map(|line| {
            // a
            line.chars()
                .map(|c| {
                    // a
                    match c {
                        '^' => Direction::N,
                        '>' => Direction::E,
                        'v' => Direction::S,
                        '<' => Direction::W,
                        _ => panic!("bad decode"),
                    }
                })
                .collect::<Vec<Direction>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    for dir in d {
        w.shift(&dir);
        println!("{}", w);
        println!("Direction {dir}");
        println!("");
    }
    0
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let map = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";

        let dirs = r"<^^>>>vv<v>>v<<";

        assert_eq!(part1(map, dirs), 11u32);
    }
}
