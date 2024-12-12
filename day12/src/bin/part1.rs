use core::num;
use std::collections::HashMap;

// row, col
static DIRECTION: [[i32; 2]; 4] = [
    [-1, 0], // N
    [0, 1],  // E
    [1, 0],  // S
    [0, -1], // W
];

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

type PlantType = char;

#[derive(Debug)]
struct RegionLabel {
    plant_type: PlantType,
    start_row: usize,
    start_col: usize,
    num_pannels: u32,
    node_count: u32,
}

#[derive(Debug)]
struct Node {
    plant_type: PlantType,
    region_id: usize,
    connection_count: u32,
    row: usize,
    col: usize,
}

fn part1(input: &str) -> u32 {
    let mut char_map: Vec<Vec<char>> = vec![];
    for (row, line) in input.lines().enumerate() {
        char_map.push(vec![]);
        for c in line.chars() {
            char_map[row].push(c);
        }
    }

    let mut regions: Vec<RegionLabel> = vec![];
    let mut node_map = HashMap::<(i32, i32), Node>::default();
    for (row, line) in char_map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            let mut new_node: Option<Node> = None;
            for d in DIRECTION {
                let search_row = row as i32 + d[0];
                let search_col = col as i32 + d[1];
                let search_key = (search_row, search_col);
                if let Some(search_node) = node_map.get_mut(&search_key) {
                    if *c == search_node.plant_type {
                        // Region identified.
                        let region_id = search_node.region_id;
                        search_node.connection_count += 1;
                        if let Some(ref mut node) = new_node {
                            node.connection_count += 1;
                        } else {
                            new_node = Some(Node {
                                plant_type: *c,
                                region_id,
                                connection_count: 1,
                                row: row as usize,
                                col: col as usize,
                            });
                        }
                    }
                }
            }

            if let Some(node) = new_node {
                node_map.insert((row as i32, col as i32), node);
            } else {
                // Add a new node.
                node_map.insert(
                    (row as i32, col as i32),
                    Node {
                        plant_type: *c,
                        region_id: regions.len(),
                        connection_count: 0,
                        row: row as usize,
                        col: col as usize,
                    },
                );

                // Start a new region.
                regions.push(RegionLabel {
                    plant_type: *c,
                    start_row: row as usize,
                    start_col: col as usize,
                    num_pannels: 0,
                    node_count: 0,
                });
            }
        }
    }

    // Have a map of all node
    // have a list of regions -- labels
    // dbg!(&node_map);
    // Compute pannels
    for (key, node) in &node_map {
        let region_id = node.region_id;
        let num_pannels = 4 - node.connection_count;
        regions[region_id].node_count += 1;
        regions[region_id].num_pannels += num_pannels;
    }

    let total_cost = regions
        .iter()
        .map(|region| {
            let p = region.num_pannels;
            let n = region.node_count;
            // cost is number of pannels * number of nodes
            let c = n * p;
            println!(
                "A region of {} plants with price {} * {} = {}",
                region.plant_type, n, p, c
            );
            c
        })
        .sum();

    dbg!(&regions);
    total_cost
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(input), 140u32);
    }

    #[test]
    fn oxo() {
        let input = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part1(input), 772u32);
    }

    #[test]
    fn larger() {
        let input = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part1(input), 772u32);
    }
}
