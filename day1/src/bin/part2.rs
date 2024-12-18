use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::combinator::{map, map_res};
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    map(
        tuple((parse_value, tag("   "), parse_value)),
        |(a, _, b)| (a, b),
    )(input)
}

fn part2(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (_remain, (a, b)) = parse_pair(line).expect("could not parse line");
        left.push(a);
        right.push(b)
    }

    left.sort();
    right.sort();

    let mut totals = vec![];
    for query in left {
        // filter right hand side and count
        let c = right.iter().filter(|x| **x == query).count();
        totals.push(query * c as u32);
    }
    totals.iter().sum()
}
