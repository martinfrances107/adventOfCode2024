use core::fmt::{self};

// (row, col)
static DIRECTION: [(i32, i32); 8] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // ES
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // WN
];

static MAS: [char; 3] = ['M', 'A', 'S'];

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
    let (count, puzzle) = part1(input);
    println!("{puzzle:?}");
    println!("{:?}", count);
}

fn part1(input: &str) -> (u32, Puzzle) {
    let mut start_positions = vec![];
    let mut lines = vec![];

    let mut output = Puzzle { grid: vec![] };
    for (row, line) in input.lines().enumerate() {
        // create row, col pairs for every x
        let line_len = line.len();
        // dbg!(line_len);
        let x_positions_row = line
            .match_indices('X')
            .map(|(col, _ch)| (row, col))
            .collect::<Vec<_>>();
        start_positions.extend_from_slice(&x_positions_row);
        let line = line.chars().collect::<Vec<_>>();
        lines.push(line);
        // TODO should optimize.
        let mut output_line = vec![];
        for _ in 0..line_len {
            output_line.push('.');
        }
        output.grid.push(output_line);
    }
    // dbg!(&x_positions);
    // let start_positions = [(4, 6)];

    let mut xmas_count = 0;
    // dbg!(&x_positions);
    for (row_idx, col_idx) in start_positions {
        // println!("starting at a new X");

        'direction_loop: for (dir_num, (row_adjust, col_adjust)) in DIRECTION.iter().enumerate() {
            // println!("Starting new direction {dir_num}");
            // start back at a know 'X' position.
            let mut candidate_col = col_idx as i32 + col_adjust;

            let mut candidate_row = row_idx as i32 + row_adjust;

            // candidate_col += col_adjust;

            let mut n_matches = 1;
            // println!("starting new xmas search");
            for wanted_char in MAS {
                // candidate_row += row_adjust;
                // Break early if row is out of bounds.
                if candidate_row < 0 {
                    // println!("row: (min) out of bounds aborting the search in this direction");
                    continue 'direction_loop;
                }

                if let Some(row) = lines.get(candidate_row as usize) {
                    // println!("row {row:?}");
                    // println!("looking for {}", wanted_char);

                    if let Some(c) = row.get(candidate_col as usize) {
                        // println!("found {c}");
                        if wanted_char == *c {
                            // println!("match yes");
                            n_matches += 1;
                            // println!("n_natches {n_matches}");
                            if n_matches == 4 {
                                // println!("outputing xmas ");
                                xmas_count += 1;
                                let mut fill_row: i32 = row_idx as i32;
                                let mut fill_col: i32 = col_idx as i32;
                                output.grid[row_idx][col_idx] = 'X';
                                for c in MAS {
                                    fill_row += row_adjust;
                                    fill_col += col_adjust;
                                    output.grid[fill_row as usize][fill_col as usize] = c;
                                }

                                continue 'direction_loop;
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

                candidate_col += col_adjust;
                candidate_row += row_adjust;
            }
        }
    }
    (xmas_count, output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn row() {
        let input = r"XMASXMASXM";
        let (count, output) = part1(&input);
        println!("output {output:#?}");
        assert_eq!(count, 2);
    }

    #[test]
    fn small() {
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

        let expected = r"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

        let (count, output) = part1(&input);
        println!("{output:#?}");
        assert_eq!(count, 18);
        // assert_eq!(expected, output.grid);
    }
}
