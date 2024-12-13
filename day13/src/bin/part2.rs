use nalgebra::{Matrix2, Vector2};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_value(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_button_a(input: &str) -> IResult<&str, (u64, u64)> {
    map(
        tuple((
            tag("Button A: X+"),
            parse_value,
            tag(", Y+"),
            parse_value,
            line_ending,
        )),
        |(_, offset_x, _, offset_y, _)| (offset_x, offset_y),
    )(input)
}

fn parse_button_b(input: &str) -> IResult<&str, (u64, u64)> {
    map(
        tuple((
            tag("Button B: X+"),
            parse_value,
            tag(", Y+"),
            parse_value,
            line_ending,
        )),
        |(_, offset_x, _, offset_y, _)| (offset_x, offset_y),
    )(input)
}

static PRIZE_OFFSET: u64 = 10000000000000_u64;
fn parse_prize(input: &str) -> IResult<&str, (u64, u64)> {
    map(
        tuple((
            tag("Prize: X="),
            parse_value,
            tag(", Y="),
            parse_value,
            line_ending,
        )),
        |(_, x, _, y, _)| (x + PRIZE_OFFSET, y + PRIZE_OFFSET),
    )(input)
}

// (A offsets) , (B offsets), (Prize Values)
fn parse_machine(input: &str) -> IResult<&str, ((u64, u64), (u64, u64), (u64, u64))> {
    tuple((parse_button_a, parse_button_b, parse_prize))(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, Vec<((u64, u64), (u64, u64), (u64, u64))>> {
    separated_list1(line_ending, parse_machine)(input)
}

fn solve(
    offsets_a: &(u64, u64),
    offsets_b: &(u64, u64),
    prizes: &(u64, u64),
) -> Option<(u64, u64)> {
    let prize_vec = Vector2::new(prizes.0 as f64, prizes.1 as f64);
    let offsets = Matrix2::new(
        offsets_a.0 as f64,
        offsets_b.0 as f64,
        offsets_a.1 as f64,
        offsets_b.1 as f64,
    );

    let det_offsets = offsets.determinant();
    if det_offsets.abs() < 1e-6 {
        println!("{det_offsets}");
        panic!("cannot invert.");
    }
    let mut a_zero = offsets.clone();

    a_zero.set_column(0, &prize_vec);
    let det_a_zero = a_zero.determinant();

    let mut a_one = offsets;
    a_one.set_column(1, &prize_vec);
    let det_a_one = a_one.determinant();

    let solution_zero = det_a_zero / det_offsets;
    let solution_one = det_a_one / det_offsets;
    println!("sol {} {}", solution_zero, solution_one);

    if solution_zero.trunc() != solution_zero {
        return None;
    }
    if solution_one.trunc() != solution_one {
        return None;
    }

    Some((solution_zero as u64, solution_one as u64))
}

fn part1(input: &str) -> u64 {
    let (_remain, machines) = parse_puzzle(input).unwrap();

    machines
        .iter()
        .map(|(offsets_a, offsets_b, prizes)| {
            // Return the cost of beating the machine.
            if let Some((a, b)) = solve(offsets_a, offsets_b, prizes) {
                // cost is 3 tokens to push A.
                // cost is 1 token to push B.
                a * 3 + b
            } else {
                // Dont play the machine.
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn parsing() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";
        let a = parse_machine(input);
        let expected = Ok(("", ((94, 34), (22, 67), (10000000008400, 10000000005400))));
        assert_eq!(a, expected);
    }

    #[test]
    fn first_and_third() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";
        let parse = parse_machine(input);
        let expected = Ok(("", ((94, 34), (22, 67), (10000000008400, 10000000005400))));
        assert_eq!(parse, expected);
        let values = parse.expect("We have values as this point");
        let ans = solve(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, None);

        let input = r"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
";
        let parse = parse_machine(input);
        let expected = Ok(("", ((17, 86), (84, 37), (10000000007870, 10000000006450))));
        assert_eq!(parse, expected);
        let values = parse.expect("We have values as this point");
        let ans = solve(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, None);
    }

    #[test]
    fn second_and_fourth_claw() {
        //This is the second claw.
        let input = r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
";
        let parse = parse_machine(input);
        let values = parse.expect("We have values as this point");
        let ans = solve(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, Some((4294967295, 4294967295)));

        let input = r"Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        let parse = parse_machine(input);
        let values = parse.expect("We have values as this point");
        let ans = solve(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, Some((4294967295, 4294967295)));
    }
}
