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
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| parse_pair(line).ok())
        .map(|(_remain, (a, b))| (a, b))
        .unzip();

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
