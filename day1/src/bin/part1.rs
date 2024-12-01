use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::combinator::{map, map_res};
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
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

fn part1(input: &str) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (_remain, (a, b)) = parse_pair(line).expect("could not parse line");
        left.push(a);
        right.push(b)
    }

    left.sort();
    right.sort();

    let b: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| {
            if right > left {
                right - left
            } else {
                left - right
            }
        })
        .sum();

    b
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), 11u32);
    }
}
