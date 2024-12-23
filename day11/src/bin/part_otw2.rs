#![feature(linked_list_cursors)]

use core::fmt::Display;
use count_digits::CountDigits;

use std::collections::LinkedList;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct Line(LinkedList<u64>);

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stone in &self.0 {
            write!(f, "{} ", stone)?;
        }
        Ok(())
    }
}
impl Line {
    fn into_ll(input: &str) -> Self {
        Self(
            input
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("a line must be all numbers"))
                .collect::<LinkedList<_>>(),
        )
    }
    fn blink(&mut self) {
        let mut cursor = self.0.cursor_front_mut();
        'walk: loop {
            let mut insert = None;
            match cursor.current() {
                Some(stone) => {
                    if *stone == 0 {
                        *stone = 1;
                    } else {
                        // Double stone
                        let num_digits = stone.count_digits();
                        if num_digits % 2 == 0 {
                            let splitter: u64 = 10u64.pow(num_digits as u32 / 2);
                            let upper = *stone / splitter;
                            let lower = *stone % splitter;
                            insert = Some(upper);
                            *stone = lower;
                        } else {
                            *stone *= 2024u64
                        }
                    }
                    if let Some(stone) = insert {
                        cursor.insert_before(stone);
                    }
                }
                None => {
                    // A fallen off the list
                    break 'walk;
                }
            }
            cursor.move_next()
        }
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(75, input));
}

fn part1(n_turns: u8, input: &str) -> usize {
    let mut line = Line::into_ll(input);

    for _ in 0..n_turns {
        line.blink();
    }

    line.0.len()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let lines = [
            // Initial arrangement:
            r"125 17",
            // After 1 blink:
            r"253000 1 7",
            // After 2 blinks:
            r"253 0 2024 14168",
            // After 3 blinks:
            r"512072 1 20 24 28676032",
            //After 4 blinks:
            r"512 72 2024 2 0 2 4 2867 6032",
            // After 5 blinks:
            r"1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
            r"2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
        ];
        for pair in lines.windows(2) {
            let first = pair[0];
            let second = pair[1];
            let mut ll0 = Line::into_ll(first);
            let ll1 = Line::into_ll(second);
            ll0.blink();
            assert_eq!(ll0, ll1);
        }
    }

    #[test]
    fn twentyFive() {
        assert_eq!(part1(6, r"125 17"), 22);
    }
}
