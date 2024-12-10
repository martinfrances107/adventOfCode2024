use core::fmt::Display;
use std::collections::{HashMap, HashSet};

use glam::i32::IVec2;

// (row, col)
static DIRECTION: [IVec2; 4] = [
    IVec2 { x: -1, y: 0 }, // N
    IVec2 { x: 0, y: 1 },  // E
    IVec2 { x: 1, y: 0 },  // S
    IVec2 { x: 0, y: -1 }, // W
];

struct Map {
    rows: Vec<Vec<char>>,
}

impl Map {
    fn new(row_size: usize, col_size: usize) -> Self {
        let mut rows = vec![];
        for row in 0..row_size {
            rows.push(vec![]);
            for _ in 0..col_size {
                rows[row].push('.');
            }
        }

        Self { rows }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    ")?;
        for row in 0..=self.rows.len() {
            write!(f, "{row} ")?;
        }
        writeln!(f)?;
        for (row_index, row) in self.rows.iter().enumerate() {
            write!(f, "{row_index}   ")?;
            for c in row {
                write!(f, "{c} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> usize {
    // A height-map keyed by position.
    let mut row_max = 0;
    let mut col_max = 0;
    let p_map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            if row > row_max {
                row_max = row;
            }
            // Note: Cannot move this into filter_map below.
            // Not because lazy evolution! But because of the move.
            // Also less comparisons this way.
            let col_count = line.len() - 1;
            if col_count > col_max {
                col_max = col_count;
            }
            line.chars().enumerate().filter_map(move |(col, c)| {
                if col > col_max {
                    col_max = col;
                }
                if c == '.' {
                    return None;
                }
                let num = c.to_digit(10).unwrap();
                Some((
                    IVec2 {
                        x: row as i32,
                        y: col as i32,
                    },
                    num,
                ))
            })
        })
        .collect::<HashMap<IVec2, u32>>();

    dbg!(row_max);
    dbg!(col_max);

    let max_steps = row_max * col_max;
    let generate_head_id = |head: IVec2| head.x * (1 + col_max as i32) + head.y;
    dbg!(max_steps);

    let mut out_map = Map::new(row_max + 1, col_max + 1);

    let heads = p_map
        .iter()
        .filter_map(|(pos, value)| {
            if *value == 0 {
                // trail head found
                Some((*pos, *value))
            } else {
                None
            }
        })
        .collect::<Vec<(IVec2, u32)>>();

    // Keyed by head_id
    let mut walkable_endpoints = HashMap::<i32, HashSet<IVec2>>::new();

    for start_point in heads.into_iter() {
        // Cannot use .enumerate() in the line above to generate a id.  The source is a hashmap
        // and so the order is not deterministic.

        let head_id = generate_head_id(start_point.0);
        let mut walk_count = 0;
        let mut unvisited_nodes = vec![start_point];
        out_map.rows[start_point.0.x as usize][start_point.0.y as usize] = '0';

        'walking: loop {
            // A pool of all new node match the hike criteria.
            let new_nodes_sprawl = unvisited_nodes
                .iter()
                .map(|node| {
                    // Newly discovered after search of 4 compass points.
                    let mut newly_discovered = vec![];
                    for offset in DIRECTION {
                        let search_pos = node.0 + offset;
                        if let Some(next_height) = p_map.get(&search_pos) {
                            let expected_height = node.1 + 1;
                            // dbg!(search_pos);
                            if *next_height == expected_height {
                                if expected_height == 0 {
                                    panic!()
                                }
                                out_map.rows[search_pos.x as usize][search_pos.y as usize] =
                                    char::from_digit(expected_height, 10).unwrap();

                                newly_discovered.push((search_pos, expected_height));
                            }
                        }
                    }
                    newly_discovered
                })
                .collect::<Vec<_>>();

            let new_nodes: Vec<_> = new_nodes_sprawl.into_iter().flatten().collect();
            for node in &new_nodes {
                if node.1 == 9 {
                    match walkable_endpoints.get_mut(&head_id) {
                        // insert(node.0)
                        Some(endpoint) => {
                            // a
                            endpoint.insert(node.0);
                        }
                        None => {
                            let endpoint_list = HashSet::from([node.0]);
                            walkable_endpoints.insert(head_id, endpoint_list);
                        }
                    };
                }
            }

            if walk_count > max_steps {
                println!("steps{walk_count} exceeds max_steps {max_steps}");
                dbg!(&new_nodes);
                panic!();
            };
            walk_count += 1;
            if new_nodes.is_empty() {
                break 'walking;
            }

            unvisited_nodes = new_nodes;
        }
    }

    walkable_endpoints
        .iter()
        .map(|list_of_endpoint| list_of_endpoint.1.len())
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first() {
        let input = r"0123
1234
8765
9876";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn score2() {
        let input = r"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        assert_eq!(part1(input), 2);
    }

    // A score of 4 because every 9 is reachable via a hiking trail except the one immediately to the left of the trailhead:
    #[test]
    fn score4() {
        let input = r"..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn multihead() {
        let input = r"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        assert_eq!(part1(input), 30);
    }

    #[test]
    fn larger() {
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        assert_eq!(part1(input), 30);
    }
}
