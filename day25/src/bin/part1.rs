use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;

use nom::combinator::map;

use nom::multi::{count, separated_list0};
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

use itertools::{Either, Itertools};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    Filled,
    Empty,
}

#[derive(Clone, Debug)]
enum Schematic {
    Lock(Vec<Vec<Element>>),
    Key(Vec<Vec<Element>>),
}

impl Schematic {
    // pin heights or lock heights
    fn heights(&self) -> Vec<u8> {
        let mut out = vec![0, 0, 0, 0, 0];
        let inerds = match self {
            Schematic::Lock(inerds) => inerds,
            Schematic::Key(inerds) => inerds,
        };

        for r in inerds {
            for (col, element) in r.iter().enumerate() {
                // Count the block in a given column.
                if *element == Element::Filled {
                    out[col] += 1;
                }
            }
        }
        out
    }
}

fn parse_row_item(input: &str) -> IResult<&str, Element> {
    map(alt((tag("#"), tag("."))), |x: &str| match x {
        "#" => Element::Filled,
        "." => Element::Empty,
        bad => {
            panic!("decoding innards: unexpected item: {bad}");
        }
    })(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Element>> {
    terminated(count(parse_row_item, 5), line_ending)(input)
}

fn parse_innards(input: &str) -> IResult<&str, Vec<Vec<Element>>> {
    count(parse_row, 5)(input)
}

fn parse_lock(input: &str) -> IResult<&str, Schematic> {
    map(delimited(parse_blocked, parse_innards, parse_open), |x| {
        println!("parse lock");
        Schematic::Lock(x)
    })(input)
}

fn parse_blocked(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("#####"), line_ending))(input)
}

fn parse_open(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("....."), line_ending))(input)
}

fn parse_key(input: &str) -> IResult<&str, Schematic> {
    map(delimited(parse_open, parse_innards, parse_blocked), |x| {
        Schematic::Key(x)
    })(input)
}

fn parse_schematic(input: &str) -> IResult<&str, Schematic> {
    alt((parse_key, parse_lock))(input)
}

fn parse_diagrams(input: &str) -> IResult<&str, Vec<Schematic>> {
    separated_list0(line_ending, parse_schematic)(input)
}

fn overlap(lock: &Schematic, key: &Schematic) -> bool {
    let l_h = lock.heights();
    let k_h = key.heights();
    for (l, h) in l_h.iter().zip(k_h) {
        if l + h < 5 {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> u32 {
    if let Ok((_, jumble)) = parse_diagrams(input) {
        let (locks, keys): (Vec<Schematic>, Vec<Schematic>) =
            jumble.into_iter().partition_map(|x| match x {
                Schematic::Lock(..) => Either::Left(x),
                Schematic::Key(..) => Either::Right(x),
            });

        let mut matches = vec![];
        for (lock, key) in locks.into_iter().cartesian_product(keys) {
            if overlap(&lock, &key) {
                println!("no match");
                matches.push(1u32);
            } else {
                println!("match");
                matches.push(0u32);
            }
        }
        dbg!(&matches);
        matches.iter().sum()
    } else {
        panic!("failed");
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r".#.#.
";
        let a = parse_row(input).expect("cannot parse row");
        println!("a {a:#?}");

        let input = r".####
.####
.####
.#.#.
.#...
";

        let (_remain, a) = parse_innards(input).expect("failed to decode innards");
        println!("a {a:#?}");
    }

    #[test]
    fn xray_lock() {
        let input = r"#####
.####
.####
.####
.#.#.
.#...
.....
";

        let is_lock = match parse_schematic(input) {
            Ok((_remain, Schematic::Lock(_))) => true,
            Ok((_remain, Schematic::Key(_))) => {
                println!("key!!!");
                true
            }
            Err(e) => {
                println!("err!!!");
                println!("{e}");
                false
            }
        };
        assert!(is_lock);

        if let Ok((_remain, xray)) = parse_schematic(input) {
            assert_eq!(xray.heights(), vec![0, 5, 3, 4, 3]);
        }
    }

    #[test]
    fn xray_key() {
        let input = r".....
#....
#....
#...#
#.#.#
#.###
#####
";

        let is_key = match parse_schematic(input) {
            Ok((_remain, Schematic::Key(_))) => true,
            Ok((_remain, Schematic::Lock(_))) => false,
            Err(e) => {
                println!("err!!!");
                println!("{e}");
                false
            }
        };

        assert!(is_key);

        if let Ok((_remain, xray)) = parse_schematic(input) {
            assert_eq!(xray.heights(), vec![5, 0, 2, 1, 3]);
        }
    }

    #[test]
    fn xray_diagram() {
        let input = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
        match parse_diagrams(input) {
            Ok((_remain, xrays)) => {
                let count = xrays.len();
                assert_eq!(count, 5);
            }
            _ => {}
        }

        assert_eq!(part1(input), 3);
    }
}
