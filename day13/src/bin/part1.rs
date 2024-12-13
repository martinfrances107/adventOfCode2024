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

fn parse_value(input: &str) -> IResult<&str, f64> {
    map_res(digit1, str::parse)(input)
}

fn parse_button_a(input: &str) -> IResult<&str, (f64, f64)> {
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

fn parse_button_b(input: &str) -> IResult<&str, (f64, f64)> {
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

fn parse_prize(input: &str) -> IResult<&str, (f64, f64)> {
    map(
        tuple((
            tag("Prize: X="),
            parse_value,
            tag(", Y="),
            parse_value,
            line_ending,
        )),
        |(_, x, _, y, _)| (x, y),
    )(input)
}

// (A offsets) , (B offsets), (Prize Values)
fn parse_machine(input: &str) -> IResult<&str, ((f64, f64), (f64, f64), (f64, f64))> {
    tuple((parse_button_a, parse_button_b, parse_prize))(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, Vec<((f64, f64), (f64, f64), (f64, f64))>> {
    separated_list1(line_ending, parse_machine)(input)
}

fn solve_within_limit(
    offsets_a: &(f64, f64),
    offsets_b: &(f64, f64),
    prizes: &(f64, f64),
) -> Option<(u32, u32)> {
    // Solve Ax = B

    // where A is a matrix of offsets
    //       B is the prizes.
    // x is  the number of button presses.

    let offsets = Matrix2::new(offsets_a.0, offsets_b.0, offsets_a.1, offsets_b.1);
    let p = Vector2::new(prizes.0, prizes.1);
    if let Some(inverse) = offsets.try_inverse() {
        let solution = inverse * p;
        // when  solution is (40.0000001, 5.0000001)
        // Solution is an integer pair
        let a = solution.x.round() as u32;
        let b = solution.y.round() as u32;

        if a > 100 || b > 100 {
            None
        } else {
            Some((a, b))
        }
    } else {
        None
    }
}

fn part1(input: &str) -> u32 {
    let (_remain, machines) = parse_puzzle(input).unwrap();

    machines
        .iter()
        .map(|(offsets_a, offsets_b, prizes)| {
            // Return the cost of beating the machine.
            let cost = if let Some((a, b)) = solve_within_limit(offsets_a, offsets_b, prizes) {
                // cost is 3 tokens to push A.
                // cost is 1 token to push B.
                a * 3 + b
            } else {
                // Dont play the machine.
                0
            };
            dbg!(cost);
            cost
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
        let expected = Ok(("", ((94., 34.), (22., 67.), (8400., 5400.))));
        assert_eq!(a, expected);
    }

    #[test]
    fn single() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";
        let parse = parse_machine(input);
        let expected = Ok(("", ((94., 34.), (22., 67.), (8400., 5400.))));
        assert_eq!(parse, expected);
        let values = parse.expect("We have values as this point");
        let ans = solve_within_limit(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, Some((80, 40)));

        let input = r"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
";
        let parse = parse_machine(input);
        let expected = Ok(("", ((17., 86.), (84., 37.), (7870., 6450.))));
        assert_eq!(parse, expected);
        let values = parse.expect("We have values as this point");
        let ans = solve_within_limit(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, Some((38, 86)));
    }

    #[test]
    fn hits_limits() {
        // The candidate solution (141, 135) exceeds the 100 press limit.
        // So its no solution.
        let input = r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
";
        let parse = parse_machine(input);
        let values = parse.expect("We have values as this point");
        let ans = solve_within_limit(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, None);

        let input = r"Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        let parse = parse_machine(input);
        let values = parse.expect("We have values as this point");
        let ans = solve_within_limit(&values.1 .0, &values.1 .1, &values.1 .2);
        assert_eq!(ans, None);
    }

    #[test]
    fn total_cost() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

        assert_eq!(part1(input), 480);
    }
}
