use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Mul,
    Add,
}
#[derive(Debug)]
struct Generator {
    active: bool,
    counter: u16,
    n_ops: u32,
}

impl Generator {
    fn new(n_ops: u32) -> Self {
        Self {
            active: true,
            counter: 2u16.pow(n_ops) - 1,
            n_ops,
        }
    }

    fn gen_list(&self) -> Vec<Operator> {
        let code_word = self.counter;
        (0..self.n_ops)
            .map(|bit_pos| {
                let mask = 1 << bit_pos;
                let bit_is_set = (mask & code_word) > 0;
                if bit_is_set {
                    Operator::Mul
                } else {
                    Operator::Add
                }
            })
            .collect::<Vec<_>>()
    }
}
impl Iterator for Generator {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.active {
            let list = self.gen_list();
            if self.counter == 0 {
                self.active = false;
            } else {
                self.counter -= 1;
            }
            Some(list)
        } else {
            None
        }
    }
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

fn part1(input: &str) -> u64 {
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

    let mut passing_total = 0;
    for line in lines {
        let Line { numbers, total } = line;

        // Setup generator
        let generate = Generator::new(numbers.len() as u32 - 1);
        'operator_loop: for operator_combination in generate {
            let mut iterator = numbers.iter();
            let mut sum = *iterator
                .next()
                .expect("Must have a lest one number in the list");
            for (next_number, operator) in iterator.zip(operator_combination) {
                match operator {
                    Operator::Add => sum += next_number,
                    Operator::Mul => sum *= next_number,
                };

                if sum == total {
                    passing_total += sum;
                    // breaking early.
                    break 'operator_loop;
                }
            }
        }
    }
    passing_total
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn generator2() {
        let mut generate = Generator::new(2);

        // 3
        assert_eq!(generate.next(), Some(vec![Operator::Mul, Operator::Mul]));
        // 2
        assert_eq!(generate.next(), Some(vec![Operator::Add, Operator::Mul]));
        // 1
        assert_eq!(generate.next(), Some(vec![Operator::Mul, Operator::Add]));
        // 0
        assert_eq!(generate.next(), Some(vec![Operator::Add, Operator::Add]));
        // list ends
        assert_eq!(generate.next(), None);
    }

    #[test]
    fn lines() {
        assert_eq!(part1("190: 10 19"), 190);

        // two solutions
        assert_eq!(part1("3267: 81 40 27"), 3267);

        assert_eq!(part1("83: 17 5"), 0);
        assert_eq!(part1("156: 15 6"), 0);
        assert_eq!(part1("7290: 6 8 6 15"), 0);

        assert_eq!(
            part1(
                r"161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
"
            ),
            0
        );

        assert_eq!(part1("292: 11 6 16 20"), 292);
    }

    #[test]
    fn example() {
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

        assert_eq!(part1(input), 3749);
    }
}
