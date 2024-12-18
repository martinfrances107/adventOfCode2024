use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut antenna_store: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let mut max_col = 0;
    let mut max_row = 0;
    for (row_index, row) in input.lines().enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            if c != '.' {
                if let Some(freq_store) = antenna_store.get_mut(&c) {
                    freq_store.insert((row_index, col_index));
                } else {
                    let mut freq_store = HashSet::new();
                    freq_store.insert((row_index, col_index));
                    antenna_store.insert(c, freq_store);
                }
            }
            if max_row < row_index as i32 {
                max_row = row_index as i32;
            }
            if max_col < col_index as i32 {
                max_col = col_index as i32;
            }
        }
    }

    // Generate map
    let mut map = HashSet::new();
    for (char, freq_store) in &antenna_store {
        if freq_store.len() == 1 {
            continue;
        }
        let fs2 = freq_store.clone();
        for ((row1, col1), (row2, col2)) in freq_store.iter().cartesian_product(fs2.iter()) {
            if *row1 == *row2 && *col1 == *col2 {
                continue;
            }

            // Compute offset
            let offset_1to2_row = *row2 as i32 - *row1 as i32;
            let offset_1to2_col = *col2 as i32 - *col1 as i32;

            let mut an1_row = *row1 as i32;
            let mut an1_col = *col1 as i32;
            let mut an2_row = *row2 as i32;
            let mut an2_col = *col2 as i32;
            for _step in 0..100 {
                // Compute antinode1 (before 1)
                an1_row -= offset_1to2_row;
                an1_col -= offset_1to2_col;

                // Compute antinode2 (beyond 2)
                an2_row += offset_1to2_row;
                an2_col += offset_1to2_col;

                // Boundary checks
                if (an1_row >= 0 && an1_row <= max_row) && (an1_col >= 0 && an1_col <= max_col) {
                    map.insert((an1_row, an1_col));
                }
                if (an2_row >= 0 && an2_row <= max_row) && (an2_col >= 0 && an2_col <= max_col) {
                    map.insert((an1_row, an1_col));
                }
            }
        }
    }
    let mut count = 0;
    // Display map
    for row in 0..=max_row {
        for col in 0..=max_col {
            // display antenna a_positions ( overwritten by #);

            let mut antenna_symbol = None;
            for (c, fs) in &antenna_store {
                if fs.contains(&(row as usize, col as usize)) {
                    antenna_symbol = Some(c);
                }
            }
            if map.contains(&(row, col)) {
                print!("#");
                count += 1;
            } else {
                if let Some(a) = antenna_symbol {
                    count += 1;
                    print!("{a}");
                }
                print!(".");
            }
        }
        println!();
    }
    count
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn simple2() {
        let input = r"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!(part1(input), 9);
    }
}
