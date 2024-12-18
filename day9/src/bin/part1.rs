fn main() {
    let input = include_str!("./input1.txt");

    let len = input.len();
    println!("size of input {len}");
    let last = input.chars().last().expect("must have a least of char");
    println!("last char {last}");

    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut dm = generate_disc_map(input);
    // Should always
    if reorder(&mut dm) {
        checksum(&dm)
    } else {
        panic!("failed to reorder");
    }
}

fn checksum(input: &[Option<u64>]) -> u64 {
    input
        .iter()
        .enumerate()
        .map(|(i, num)| match num {
            Some(num) => i as u64 * *num,
            None => 0,
        })
        .sum()
}

fn reorder(line: &mut [Option<u64>]) -> bool {
    let len = line.len();
    for _ in 0..len {
        if !shuffle(line) {
            return true;
        }
    }
    panic!("Advanced beyond point where shuffling should have stopped");
}

fn generate_disc_map(input: &str) -> Vec<Option<u64>> {
    let a = input.chars().collect::<Vec<_>>();

    let fragments: Vec<Vec<Option<u64>>> = a
        .chunks(2)
        .enumerate()
        .map(|(block_id, pair_iter)| {
            // decode

            let mut fragment: Vec<Option<u64>> = vec![];
            // Panics if block size is not seem.
            let block_size: u32 = pair_iter
                .first()
                .unwrap()
                .to_digit(10)
                .expect("must decode block size");
            for _ in 0..block_size {
                fragment.push(Some(block_id as u64))
            }

            if let Some(free_space_char) = pair_iter.get(1) {
                let free_space = free_space_char.to_digit(10).unwrap();
                let f_len = fragment.len();
                let additional_space = free_space as usize;
                fragment.resize(f_len + additional_space, None);
            }

            fragment
        })
        .collect();

    fragments.into_iter().flatten().collect()
}

fn shuffle(input: &mut [Option<u64>]) -> bool {
    let first_blank = input.iter().position(|x| x.is_none());
    let last_num = input.iter().rposition(|x| x.is_some());
    match (first_blank, last_num) {
        (Some(first), Some(last)) => {
            if first == last {
                panic!("Cannot have both blank and number in the same position");
            }
            if first > last {
                // Cannot shuffle as the first blank is now to the right of the numbers.
                false
            } else {
                input.swap(first, last);
                true
            }
        }
        (None, Some(_last)) => {
            panic!("no longer good");
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
        let expected = "0..111....22222"
            .chars()
            .map(|c| {
                // h
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => None,
                }
            })
            .collect::<Vec<Option<u64>>>();

        assert_eq!(generate_disc_map(input), expected);

        let input = "90909";
        let expected = "000000000111111111222222222"
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => None,
                }
            })
            .collect::<Vec<Option<u64>>>();
        assert_eq!(generate_disc_map(input), expected);

        let input = r"2333133121414131402";
        let expected = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => None,
                }
            })
            .collect::<Vec<Option<u64>>>();
        assert_eq!(generate_disc_map(input), expected);
    }

    #[test]
    fn shuffle_short() {
        let steps = [
            "0..111....22222",
            "02.111....2222.",
            "022111....222..",
            "0221112...22...",
            "02211122..2....",
            "022111222.....x",
        ];

        for pair in steps.windows(2) {
            match pair {
                [first, second] => {
                    let mut input = first
                        .chars()
                        .map(|c| {
                            // a
                            let num = c.to_digit(10);
                            match num {
                                Some(n) => Some(n as u64),
                                None => None,
                            }
                        })
                        .collect::<Vec<Option<u64>>>();
                    let expected = second
                        .chars()
                        .map(|c| {
                            // a
                            let num = c.to_digit(10);
                            match num {
                                Some(n) => Some(n as u64),
                                None => None,
                            }
                        })
                        .collect::<Vec<Option<u64>>>();
                    let did_shuffle = shuffle(&mut input);
                    assert!(did_shuffle);
                    // println!("input {:#?}", &input);
                    assert_eq!(input, expected);
                }
                [_one] => {
                    panic!("mut get a pair of steps");
                }
                [] => {
                    panic!("mut get more than one");
                }
                _ => {
                    panic!("more than 2")
                }
            }
        }
    }

    #[test]
    fn shuffle_long() {
        let steps = [
            "00...111...2...333.44.5555.6666.777.888899",
            "009..111...2...333.44.5555.6666.777.88889.",
            "0099.111...2...333.44.5555.6666.777.8888..",
            "00998111...2...333.44.5555.6666.777.888...",
            "009981118..2...333.44.5555.6666.777.88....",
            "0099811188.2...333.44.5555.6666.777.8.....",
            "009981118882...333.44.5555.6666.777.......",
            "0099811188827..333.44.5555.6666.77........",
            "00998111888277.333.44.5555.6666.7.........",
            "009981118882777333.44.5555.6666...........",
            "009981118882777333644.5555.666............",
            "00998111888277733364465555.66.............",
            "0099811188827773336446555566..............",
        ];

        for pair in steps.windows(2) {
            match pair {
                [first, second] => {
                    let mut input = first
                        .chars()
                        .map(|c| {
                            // a
                            let num = c.to_digit(10);
                            match num {
                                Some(n) => Some(n as u64),
                                None => None,
                            }
                        })
                        .collect::<Vec<Option<u64>>>();
                    let expected = second
                        .chars()
                        .map(|c| {
                            // a
                            let num = c.to_digit(10);
                            match num {
                                Some(n) => Some(n as u64),
                                None => None,
                            }
                        })
                        .collect::<Vec<Option<u64>>>();
                    let did_shuffle = shuffle(&mut input);
                    assert!(did_shuffle);
                    // println!("input {:#?}", &input);
                    assert_eq!(input, expected);
                }
                [_one] => {
                    panic!("mut get a pair of steps");
                }
                [] => {
                    panic!("mut get more than one");
                }
                _ => {
                    panic!("more than 2")
                }
            }
        }
    }

    #[test]
    fn reorder_check() {
        let line = "12345";
        let mut dm = generate_disc_map(line);
        let expected = "022111222......"
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => None,
                }
            })
            .collect::<Vec<Option<u64>>>();
        assert!(reorder(&mut dm));
        assert_eq!(dm, expected);

        let mut line = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => None,
                }
            })
            .collect::<Vec<Option<u64>>>();
        let expected = "0099811188827773336446555566.............."
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => None,
                }
            })
            .collect::<Vec<Option<u64>>>();
        assert!(reorder(&mut line));
        assert_eq!(line, expected);
    }

    #[test]
    fn checksum_test() {
        let line = "0099811188827773336446555566"
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => panic!("should always decode here"),
                }
            })
            .collect::<Vec<Option<u64>>>();
        // println!("line {line:#?}");
        assert_eq!(checksum(&line), 1928);
    }

    #[test]
    fn full_monty() {
        let input = r"2333133121414131402";
        assert_eq!(part1(input), 1928);
    }
}
