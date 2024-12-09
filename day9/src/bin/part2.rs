use core::fmt::Display;

fn main() {
    let input = include_str!("./input1.txt");

    let len = input.len();
    println!("size of input {len}");
    let last = input.chars().last().expect("must have a least of char");
    println!("last char {last}");

    println!("{:?}", part2(input));
}

#[derive(Debug)]
struct File {
    block_id: usize,
    start_addr: usize,
    size: usize,
}

#[derive(Debug)]
struct Blank {
    start_addr: usize,
    size: usize,
}

#[derive(Debug)]
enum DiscEntry {
    B(Blank),
    F(File),
}

impl Display for DiscEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscEntry::B(b) => {
                for _ in 0..b.size {
                    write!(f, ".")?;
                }
            }
            DiscEntry::F(file) => {
                for _ in 0..file.size {
                    write!(f, "{}", file.block_id)?;
                }
            }
        }
        Ok(())
    }
}

impl DiscEntry {
    fn checksum_vec(&self) -> Vec<Option<u64>> {
        let mut out = vec![];
        match self {
            DiscEntry::B(b) => {
                for _i in 0..b.size {
                    out.push(None)
                }
            }
            DiscEntry::F(f) => {
                for _i in 0..f.size {
                    out.push(Some(f.block_id as u64))
                }
            }
        }
        out
    }
}
impl Display for DiscMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for de in &self.0 {
            write!(f, "{de}")?;
        }
        Ok(())
    }
}
impl DiscMap {
    fn checksum_vec(&self) -> Vec<Option<u64>> {
        let mut fragments = vec![];

        for de in &self.0 {
            fragments.push(de.checksum_vec())
        }
        fragments.into_iter().flatten().collect()
    }
}
struct DiscMap(Vec<DiscEntry>);

fn part2(input: &str) -> u64 {
    let dm = generate_disc_map(input);

    // Sorted by start addr.
    let mut spaces = vec![];
    // Sorted by file id.
    let mut files = vec![];
    for de in dm.0 {
        match de {
            DiscEntry::B(b) => spaces.push(b),
            DiscEntry::F(f) => files.push(f),
        }
    }

    // Moved file with a high file id first.
    // b before a is reverse ordering!
    files.sort_by(|a, b| b.block_id.cmp(&a.block_id));

    // Blanks with a low start addr first.
    spaces.sort_by(|a, b| a.start_addr.cmp(&b.start_addr));

    // for every file  try to place in the first blank first.
    for file in &mut files {
        let mut hole = None;
        let mut index_to_remove = None;
        'space_search: for (i, space) in spaces.iter_mut().enumerate() {
            // Will file fit into space.
            if file.size <= space.size {
                if space.start_addr > file.start_addr {
                    break 'space_search;
                }
                // hole left behind by moved file.
                hole = Some(Blank {
                    start_addr: file.start_addr,
                    size: file.size,
                });

                // Move the file to a new start addr.
                file.start_addr = space.start_addr;

                if file.size == space.size {
                    // The blank space is consumed.
                    index_to_remove = Some(i);
                } else {
                    // Shift up and shrink size of filled space.
                    space.start_addr += file.size;
                    space.size -= file.size;
                }
                // A move has occurred. stop space search.
                break 'space_search;
            }
        }

        // Removed comsumed space.
        if let Some(index) = index_to_remove {
            spaces.remove(index);
        }
        // if the file was moved then there is a hole.
        if let Some(new_blank) = hole {
            spaces.push(new_blank);
            spaces.sort_by(|a, b| a.start_addr.cmp(&b.start_addr));
        }
    }

    let mut ordered_de = vec![];
    for file in files {
        ordered_de.push(DiscEntry::F(file));
    }
    for space in spaces {
        ordered_de.push(DiscEntry::B(space));
    }

    ordered_de.sort_by(|de_a, de_b| match (de_a, de_b) {
        (DiscEntry::B(a), DiscEntry::B(b)) => a.start_addr.cmp(&b.start_addr),
        (DiscEntry::B(a), DiscEntry::F(b)) => a.start_addr.cmp(&b.start_addr),
        (DiscEntry::F(a), DiscEntry::B(b)) => a.start_addr.cmp(&b.start_addr),
        (DiscEntry::F(a), DiscEntry::F(b)) => a.start_addr.cmp(&b.start_addr),
    });

    let ordered_dm = DiscMap(ordered_de);

    println!("str {}", ordered_dm);
    let checksum_vec = ordered_dm.checksum_vec();

    checksum(&checksum_vec)
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

fn generate_disc_map(input: &str) -> DiscMap {
    let a = input.chars().collect::<Vec<_>>();
    let mut addr = 0;
    let fragments: Vec<Vec<DiscEntry>> = a
        .chunks(2)
        .enumerate()
        .map(|(block_id, pair_iter)| {
            // decode

            let mut fragment: Vec<DiscEntry> = vec![];
            // Panics if block size is not seem.
            let block_size: u32 = pair_iter
                .first()
                .unwrap()
                .to_digit(10)
                .expect("must decode block size");

            fragment.push(DiscEntry::F(File {
                start_addr: addr,
                block_id,
                size: block_size as usize,
            }));
            addr += block_size as usize;

            if let Some(free_space_char) = pair_iter.get(1) {
                let free_space = free_space_char.to_digit(10).unwrap();
                fragment.push(DiscEntry::B(Blank {
                    start_addr: addr,
                    size: free_space as usize,
                }));
                addr += free_space as usize;
            }

            fragment
        })
        .collect();

    let disc_entries = fragments.into_iter().flatten().collect();
    DiscMap(disc_entries)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn checksum_test() {
        let line = "00992111777.44.333....5555.6666.....8888.."
            .chars()
            .map(|c| {
                // a
                let num = c.to_digit(10);
                match num {
                    Some(n) => Some(n as u64),
                    None => Some(0),
                }
            })
            .collect::<Vec<Option<u64>>>();
        assert_eq!(checksum(&line), 2858);
    }

    #[test]
    fn full_monty2() {
        let input = r"2333133121414131402";
        assert_eq!(part2(input), 2859);
    }
}
