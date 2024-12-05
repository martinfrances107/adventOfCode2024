use core::panic;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{sequence::tuple, IResult};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_rule_pair(input: &str) -> IResult<&str, (u32, u32)> {
    map(
        tuple((parse_value, tag("|"), parse_value, line_ending)),
        |(a, _, b, _)| (a, b),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (remain, (pair, _newline)) = many_till(parse_rule_pair, line_ending)(input)?;
    Ok((remain, pair))
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(tag(","), parse_value), line_ending)(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(parse_update)(input)
}

fn part1(input: &str) -> u32 {
    if let Ok((updates_str, rules)) = parse_rules(input) {
        println!("n_ruiles {}", rules.len());
        // println!("rules {:?}", rules);
        if let Ok((_remain, updates)) = parse_updates(updates_str) {
            println!("n_updates {}", updates.len());

            return updates
                .iter()
                // filter out failing updates.
                .filter(|update| {
                    for item in update.iter() {
                        let count = update.iter().filter(|x| **x == *item).count();
                        if count != 1 {
                            dbg!(update);
                            panic!("update item is not unique");
                        }
                    }
                    for (l, r) in &rules {
                        // Must contain both L and R.
                        let l_pos = update.iter().position(|x| *l == *x);
                        let r_pos = update.iter().position(|x| *r == *x);
                        match (l_pos, r_pos) {
                            (Some(l_index), Some(r_index)) => {
                                if r_index == l_index {
                                    panic!("unexpected Item in the bagging area.");
                                }
                                if r_index < l_index {
                                    // Reject: L must come before R.
                                    return false;
                                }
                            }
                            _ => {
                                // rules does not apply.
                                // Move onto next rule
                            }
                        }
                    }
                    // this algorithm  short circuit here.. so return true
                    // println!("passing update {:#?}", update);
                    true
                })
                .map(|update| {
                    let mid = &update.len() / 2;
                    if update.len() % 2 == 0 {
                        dbg!(update.len());
                        dbg!(mid);
                        dbg!(update);
                        panic!("update is an odd number of items");
                    }
                    update[mid]
                })
                .sum();
        }
        {
            println!("failed to parse updates");
            panic!()
        }
    }

    println!("failed to parse rules");
    panic!();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn rules() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

";

        let expected = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        assert_eq!(parse_rules(input), Ok(("", expected)));
    }

    #[test]
    fn updates() {
        let input = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let expected: Vec<Vec<u32>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(parse_updates(input), Ok(("", expected)));
    }

    #[test]
    fn example() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part1(input), 143);
    }
}
