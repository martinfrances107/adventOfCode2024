use core::panic;

static CHAR_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut dm = generate_disc_map(input);
    // Should always
    if reorder(&mut dm) {
        println!("{dm:#?}");
        checksum(&dm)
    } else {
        panic!("failed to reorder");
    }
}

fn checksum(input: &[char]) -> u64 {
    input
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let n: u64 = String::from(*c).parse().expect("must have number");
            n * i as u64
        })
        .sum()
}

fn reorder(line: &mut Vec<char>) -> bool {
    let len = line.len();
    for _ in 0..len {
        if shuffle(line) {
            line.pop();
        } else {
            return true;
        }
    }
    panic!("Advanced beyond point where shuffling should have stopped");
}

fn generate_disc_map(input: &str) -> Vec<char> {
    let a = input
        .chars()
        .map(|c| {
            let n: u8 = String::from(c).parse().expect("must have number");
            n
        })
        .collect::<Vec<_>>();

    let fragments: Vec<Vec<char>> = a
        .chunks(2)
        .enumerate()
        .map(|(block_id, pair_iter)| {
            // decode
            let block_id_char = block_id.to_string().chars().next().unwrap();
            if !CHAR_DIGITS.contains(&block_id_char) {
                panic!("bad decode of block_id_char");
            }
            let mut fragment = vec![];
            match pair_iter {
                [block_size, free_space] => {
                    for _ in 0..*block_size {
                        fragment.push(block_id_char)
                    }

                    let f_len = fragment.len();
                    let additional_space = *free_space as usize;
                    fragment.resize(f_len + additional_space, '.');
                }
                [block_size] => {
                    for _ in 0..*block_size {
                        fragment.push(block_id_char)
                    }
                }
                _ => {
                    panic!("none or more than 2");
                }
            }

            fragment
        })
        .collect();

    fragments.into_iter().flatten().collect()
}

fn shuffle(input: &mut [char]) -> bool {
    let first_blank = input.iter().position(|x| *x == '.');
    let last_num = input.iter().rposition(|x| *x != '.');
    match (first_blank, last_num) {
        (Some(first), Some(last)) => {
            input.swap(first, last);
            true
        }
        (None, Some(_last)) => {
            // shuffling complete.
            // all dots removed.
            false
        }
        (Some(_first), None) => {
            panic!("bad failure mode");
        }
        (None, None) => {
            // No shuffle possible
            panic!("both should never fail")
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"12345";
        let expected = "0..111....22222".chars().collect::<Vec<char>>();
        assert_eq!(generate_disc_map(input), expected);
        let input = r"2333133121414131402";
        let expected = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .collect::<Vec<char>>();
        assert_eq!(generate_disc_map(input), expected);
    }

    #[test]
    fn shuffle09() {
        let mut line = r"00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .collect::<Vec<char>>();
        assert!(shuffle(&mut line));
        let expected = "009..111...2...333.44.5555.6666.777.88889."
            .chars()
            .collect::<Vec<char>>();
        assert_eq!(line, expected);
    }

    #[test]
    fn reorder_check() {
        let mut line = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .collect::<Vec<_>>();
        let expected = "0099811188827773336446555566".chars().collect::<Vec<_>>();
        assert!(reorder(&mut line));
        assert_eq!(line, expected);
    }

    #[test]
    fn checksum_test() {
        let line = "0099811188827773336446555566".chars().collect::<Vec<_>>();
        assert_eq!(checksum(&line), 1928);
    }

    #[test]
    fn full_monty() {
        let input = r"2333133121414131402";
        assert_eq!(part1(input), 1928);
    }
}
