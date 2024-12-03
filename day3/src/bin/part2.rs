use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::{map, map_res};
use nom::multi::{many1, many_till};
use nom::{sequence::tuple, IResult};

#[derive(Debug)]
enum Instr {
    Start,
    Mul(u32, u32),
    Stop,
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    map(tuple((parse_value, tag(","), parse_value)), |(a, _, b)| {
        (a, b)
    })(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instr> {
    map(
        tuple((tag("mul("), parse_pair, tag(")"))),
        |(_head, (x, y), _tail)| Instr::Mul(x, y),
    )(input)
}

fn parse_stop(input: &str) -> IResult<&str, Instr> {
    map(tag("don't()"), |_| Instr::Stop)(input)
}

fn parse_start(input: &str) -> IResult<&str, Instr> {
    map(tag("do()"), |_| Instr::Start)(input)
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    let res = many_till(
        anychar,
        alt((
            map(parse_start, |i| i),
            map(parse_stop, |i| i),
            map(parse_mul, |i| i),
        )),
    )(input)?;
    let (remain, (_junk, instr)) = res;
    Ok((remain, instr))
}

fn part2(input: &str) -> u32 {
    let a = many1(parse_instr)(input);
    if let Ok((_tail, instrs)) = a {
        let mut active = true;
        let mut sum = 0;
        for i in instrs {
            match i {
                Instr::Mul(x, y) => {
                    if active {
                        sum += x * y;
                    }
                }
                Instr::Stop => {
                    active = false;
                }
                Instr::Start => {
                    active = true;
                }
            }
        }
        sum
    } else {
        panic!("must investigate");
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn simple() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part2(input), 161);
    }
}
