use core::fmt::{self};

// (row, col)
static DIRECTION: [[(i32, i32); 2]; 4] = [
    // top left to bottom right.
    [(-1_i32, -1_i32), (1_i32, 1_i32)],
    // top right to bottom left
    [(-1_i32, 1_i32), (1_i32, -1_i32)],
    // bottom right to top left
    [(1_i32, 1_i32), (-1_i32, -1_i32)],
    // bottom left to top right
    [(1_i32, -1_i32), (-1_i32, 1_i32)],
];

static MS: [char; 2] = ['M', 'S'];

#[derive(Clone)]
struct Puzzle {
    grid: Vec<Vec<char>>,
}

impl std::fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let (count, puzzle) = part2(input);
    println!("{puzzle:?}");
    println!("{:?}", count);
}

fn part2(input: &str) -> (u32, Puzzle) {
    let mut start_positions = vec![];
    let mut lines = vec![];

    let mut output = Puzzle { grid: vec![] };
    for (row, line) in input.lines().enumerate() {
        // create row, col pairs for every x
        let line_len = line.len();
        let a_positions = line
            .match_indices('A')
            .map(|(col, _ch)| (row, col))
            .collect::<Vec<_>>();
        start_positions.extend_from_slice(&a_positions);
        let line = line.chars().collect::<Vec<_>>();
        lines.push(line);
        // TODO should optimize.
        let mut output_line = vec![];
        for _ in 0..line_len {
            output_line.push('.');
        }
        output.grid.push(output_line);
    }

    // Counts num of observed x-mas.
    let mut x_count = 0;
    // dbg!(&x_positions);
    for (row_idx, col_idx) in start_positions {
        // println!("starting at a new X");
        let mut diagonals = 0;
        'direction_loop: for offsets in DIRECTION.iter() {
            let mut ms_matches = 0;
            for (wanted_char, (row_adjust, col_adjust)) in MS.iter().zip(offsets) {
                let candidate_row = row_idx as i32 + row_adjust;

                // Break early if row is out of bounds.
                if candidate_row < 0 {
                    // println!("row: (min) out of bounds aborting the search in this direction");
                    continue 'direction_loop;
                }

                if let Some(row) = lines.get(candidate_row as usize) {
                    let candidate_col = col_idx as i32 + col_adjust;
                    if let Some(c) = row.get(candidate_col as usize) {
                        // println!("wanted {wanted_char} found  {c}");
                        if *wanted_char == *c {
                            // println!("match yes");
                            ms_matches += 1;
                            // println!("ms_matches {ms_matches}");
                            if ms_matches == 2 {
                                diagonals += 1;
                            }
                        } else {
                            // println!("does not match check next direction");
                            continue 'direction_loop;
                        }
                    } else {
                        // println!("col: out of bound");
                        continue 'direction_loop;
                    }
                } else {
                    // println!("row: (max) out of bounds aborting the search in this direction");
                    continue 'direction_loop;
                }
            }

            if diagonals == 2 {
                x_count += 1;
                output.grid[row_idx][col_idx] = 'A';
                continue 'direction_loop;
            }
        }
    }
    (x_count, output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn x_small() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let (count, output) = part2(&input);
        println!("{output:#?}");
        assert_eq!(count, 9);
    }
}
