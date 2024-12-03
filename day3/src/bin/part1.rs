use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};

use nom::combinator::{map, map_res};
use nom::multi::{many1, many_till};
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    map(tuple((parse_value, tag(","), parse_value)), |(a, _, b)| {
        (a, b)
    })(input)
}

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    map(
        tuple((tag("mul("), parse_pair, tag(")"))),
        |(_head, (x, y), _tail)| (x, y),
    )(input)
}

fn parse_instr(input: &str) -> IResult<&str, (u32, u32)> {
    let (remain, (_junk, instruction)) = many_till(anychar, parse_mul)(input)?;
    Ok((remain, instruction))
}

fn part1(mut input: &str) -> u32 {
    let a = many1(parse_instr)(input);
    if let Ok((tail, instrs)) = a {
        println!("instruct {instrs:#?}");
        instrs.iter().map(|(x, y)| x * y).sum()
    } else {
        panic!("failed to parse");
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn single() {
        let input = r"    mul(2,3)";
        assert_eq!(find_mul(input), Ok(("mul(2,3)", "    ")));

        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            find_mul(input),
            Ok((
                "mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
                "x"
            ))
        );

        let input = r"%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        if let Ok((start_with_mul, head)) = find_mul(input) {
            println!("starts with {start_with_mul}");
            println!("head {head}");
            assert_eq!(
                parse_mul(start_with_mul),
                Ok(("+mul(32,64]then(mul(11,8)mul(8,5))", (5, 5)))
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_mul() {
        let input = r"mul(2,3)";
        assert_eq!(parse_mul(input), Ok(("", (2, 3))));
    }

    #[test]
    fn simple() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }
}
