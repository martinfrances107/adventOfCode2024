fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let dm = generate_disc_map(input);
    // Should always
    let reordered = reorder(dm);

    // Compute checksum

    checksum(&reordered)
}

fn checksum(input: &str) -> u64 {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let n: u64 = String::from(c).parse().expect("must have number");
            n * i as u64
        })
        .sum()
}
fn reorder(mut line: String) -> String {
    let len = line.len();
    for _ in 0..len {
        match shuffle(&line) {
            Some(next) => {
                // println!("next {next}");
                let len = line.len() - 1;
                line = next[0..len].to_string();
            }
            None => return line,
        }
    }
    panic!("Advanced beyond point where shuffling should have stopped");
}

fn generate_disc_map(input: &str) -> String {
    let a = input
        .chars()
        .map(|c| {
            let n: u8 = String::from(c).parse().expect("must have number");
            n
        })
        .collect::<Vec<_>>();

    let fragments: Vec<String> = a
        .chunks(2)
        .enumerate()
        .map(|(block_id, pair_iter)| {
            // decode
            let block_id_str = block_id.to_string();
            let mut fragment = String::from("");
            match pair_iter {
                [block_size, free_space] => {
                    for _ in 0..*block_size {
                        fragment.push_str(&block_id_str)
                    }
                    for _ in 0..*free_space {
                        fragment.push('.');
                    }
                }
                [block_size] => {
                    for _ in 0..*block_size {
                        fragment.push_str(&block_id_str)
                    }
                }
                _ => {
                    panic!("none or more than 2");
                }
            }

            fragment
        })
        .collect();
    let mut disc_map = String::new();
    for frag in fragments {
        disc_map.push_str(&frag);
    }

    disc_map
}

fn shuffle(input: &str) -> Option<String> {
    let first_blank = input.find('.');
    let last_num = input.rfind(|x| x != '.');
    match (first_blank, last_num) {
        (Some(first), Some(last)) => {
            // swap
            let mut chars: Vec<_> = input.chars().collect();
            chars.swap(first, last);
            Some(chars.into_iter().collect())
        }
        _ => {
            // No shuffle possible
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"12345";
        assert_eq!(generate_disc_map(input), "0..111....22222");
        let input = r"2333133121414131402";
        assert_eq!(
            generate_disc_map(input),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn shuffle09() {
        let input = r"00...111...2...333.44.5555.6666.777.888899";
        let shuffled = shuffle(&input);
        assert_eq!(
            shuffled,
            Some(String::from("009..111...2...333.44.5555.6666.777.88889."))
        );
    }

    #[test]
    fn reorder_check() {
        let dm = String::from("00...111...2...333.44.5555.6666.777.888899");
        assert_eq!(reorder(dm), "0099811188827773336446555566");
    }

    #[test]
    fn checksum_test() {
        let line = String::from("0099811188827773336446555566");
        assert_eq!(checksum(&line), 1928);
    }

    #[test]
    fn full_monty() {
        let input = r"2333133121414131402";
        assert_eq!(part1(input), 1928);
    }
}
