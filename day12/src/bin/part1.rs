use core::num;
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone)]
struct Node {
    plant_type: PlantType,
    connections: Vec<(i32, i32)>,
    pos: (i32, i32),
}

#[derive(Debug)]
struct Region {
    plantType: PlantType,
    nodes: HashSet<(i32, i32)>,
}
fn part1(input: &str) -> u32 {
    // Bare in the sense connections unfilled.
    let mut node_map_bare = HashMap::<(i32, i32), Node>::default();
    // Define unconnected node map.
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            node_map_bare.insert(
                (row as i32, col as i32),
                Node {
                    plant_type: c,
                    connections: vec![],
                    pos: (row as i32, col as i32),
                },
            );
        }
    }

    let mut node_map = node_map_bare.clone();
    // Make all connections.
    for ((row, col), node) in node_map.iter_mut() {
        for d in DIRECTION {
            let search_row = row + d[0];
            let search_col = col + d[1];
            let search_key = (search_row, search_col);
            if let Some(search_node) = node_map_bare.get(&search_key) {
                if node.plant_type == search_node.plant_type {
                    // Edge detected
                    // println!(
                    //     "edge detected ({},{}) -> ({}, {})",
                    //     row, col, search_row, search_col
                    // );
                    node.connections.push(search_key);
                }
            }
        }
    }

    let node_map_a = node_map
        .iter()
        .filter(|(pos, n)| {
            // a
            n.plant_type == 'A'
        })
        .collect::<Vec<_>>();

    dbg!(&node_map_a);
    let mut regions: Vec<Region> = vec![];

    // While there are node in the node_map
    // removed them one by one and add them to the correct regions.
    for (active_pos, active_node) in node_map.drain() {
        // Use connections to locate the region.
        let mut place_found = None;
        for connection in &active_node.connections {
            // Where should the node be placed.
            'region_search: for (idx, region) in regions.iter().enumerate() {
                if region.nodes.contains(connection) {
                    place_found = Some(idx);
                    break 'region_search;
                }
            }
        }
        if let Some(region_index) = place_found {
            let mut connection_pool: HashSet<(i32, i32)> = HashSet::default();
            // prime the connection pool
            for connection in active_node.connections {
                // get the node
                connection_pool.insert(connection);
            }
            'connections_walk: loop {
                // check all items in the connection pool until no more are added.
                let mut more_found = false;
                for connection in connection_pool.iter() {
                    if let Some(cn) = node_map.get(connection) {
                        if connection_pool.contains(&cn.pos) {
                            more_found = true;
                        }
                    }
                }
            }
        } else {
            // Start a new region
            let mut region = Region {
                plantType: active_node.plant_type,
                nodes: HashSet::default(),
            };
            region.nodes.insert(active_pos);
            for connection in active_node.connections {
                region.nodes.insert(connection);
            }
            regions.push(region);
        }
    }

    // Have a map of all node
    // have a list of regions -- labels
    // dbg!(&node_map);
    // Compute pannels
    // for (key, node) in &node_map {
    //     let region_id = node.region_id;
    //     let num_pannels = 4 - node.connection_count;
    //     regions[region_id].node_count += 1;
    //     regions[region_id].num_pannels += num_pannels;
    // }

    // let total_cost = regions
    //     .iter()
    //     .map(|region| {
    //         let p = region.num_pannels;
    //         let n = region.node_count;
    //         // cost is number of pannels * number of nodes
    //         let c = n * p;
    //         println!(
    //             "A region of {} plants with price {} * {} = {}",
    //             region.plant_type, n, p, c
    //         );
    //         c
    //     })
    //     .sum();

    let r_A = regions
        .iter()
        .filter(|r| r.plantType == 'A')
        .collect::<Vec<_>>();
    dbg!(&r_A);

    0
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
