use itertools::Itertools;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operator {
    Mul,
    Add,
    Merge,
}

impl TryFrom<i32> for Operator {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operator::Add),
            1 => Ok(Operator::Mul),
            2 => Ok(Operator::Merge),
            x => Err(format!("Number must be zero two or two, found {x}")),
        }
    }
}

static OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Merge];

// Sequences of n possible Operators.
// Considering list of 2 operators
// there are 3*3 9 sequences
// Considering list of 3 operators
// There are 3*3*3
fn sequences(n: usize) -> Vec<Vec<Operator>> {
    (0..n)
        .map(|_| OPERATORS.clone())
        .collect::<Vec<_>>()
        .into_iter()
        .multi_cartesian_product()
        .collect::<Vec<_>>()
}

fn parse_value(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), parse_value)(input)
}

struct Line {
    total: u64,
    numbers: Vec<u64>,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((parse_value, tag(": "), parse_list)),
        |(total, _, numbers)| Line { total, numbers },
    )(input)
}

fn part2(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|line| parse_line(line))
        .filter_map(|res| {
            if let Ok((_remain, a)) = res {
                Some(a)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    'line_loop: for line in lines {
        let Line { numbers, total } = line;
        let n_operators = numbers.len() - 1;
        let op_lists = sequences(n_operators);
        // dbg!(&op_lists);
        for op_list in &op_lists {
            let mut iter = numbers.iter();
            let mut line_sum = *iter.next().expect("must have at least one number");
            // println!("first number {line_sum}");
            // dbg!(line_sum);
            // dbg!(&op_list);
            for (num, operator) in iter.zip(op_list) {
                match operator {
                    Operator::Add => line_sum += num,
                    Operator::Mul => line_sum *= num,
                    Operator::Merge => {
                        let sum_string = line_sum.to_string();
                        let new_sum = format!("{sum_string}{num}");
                        line_sum = new_sum.parse::<u64>().unwrap();
                    }
                }
            }
            if line_sum == total {
                sum += line_sum;
                continue 'line_loop;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn lines() {
        assert_eq!(part2("190: 10 19"), 190);

        // // two solutions
        assert_eq!(part2("3267: 81 40 27"), 3267);
        assert_eq!(part2("83: 17 5"), 0);
        assert_eq!(part2("156: 15 6"), 156);
        assert_eq!(part2("7290: 6 8 6 15"), 7290);
        assert_eq!(part2("161011: 16 10 13"), 0);
        assert_eq!(part2("192: 17 8 14"), 192);
        assert_eq!(part2("192: 17 8 14"), 192);
        assert_eq!(part2("21037: 9 7 18 13"), 0);
        assert_eq!(part2("292: 11 6 16 20"), 292);
    }

    #[test]
    fn exampleMAMrege() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

        assert_eq!(part2(input), 11387);
    }
}
