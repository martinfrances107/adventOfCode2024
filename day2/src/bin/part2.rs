use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn parse_level(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_report(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(" "), parse_level)(input)
}

fn is_ascending_undamped(report: &Vec<u32>) -> bool {
    let mut last = u32::MIN;
    let mut is_ascending = true;
    for level in report {
        if *level <= last {
            is_ascending = false;
        }
        last = *level;
    }
    is_ascending
}

fn is_descending_undamped(report: &Vec<u32>) -> bool {
    let mut last = u32::MAX;
    let mut is_descending = true;
    for level in report {
        if *level >= last {
            is_descending = false;
        }
        last = *level;
    }

    is_descending
}

fn big_gaps_undamped(report: &[u32]) -> bool {
    let mut big_gaps = false;
    for (a, b) in report.iter().tuple_windows() {
        if a.abs_diff(*b) > 3 {
            big_gaps = true;
        }
    }
    big_gaps
}

#[inline]
fn is_safe(report: &Vec<u32>) -> bool {
    !big_gaps_undamped(report) && (is_ascending_undamped(report) || is_descending_undamped(report))
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            // filters only good reports.
            parse_report(line).ok()
        })
        .filter(|(_remain, report)| {
            if !is_safe(report) {
                for index in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(index);
                    if is_safe(&new_report) {
                        return true;
                    }
                }
                false
            } else {
                true
            }
        })
        .count()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example2() {
        let input = r"7 6 4 2 1";
        assert_eq!(part2(input), 1);

        let input = r"1 2 7 8 9";
        assert_eq!(part2(input), 0);

        let input = r"9 7 6 2 1";
        assert_eq!(part2(input), 0);

        let input = r"1 3 2 4 5";
        assert_eq!(part2(input), 1);

        let input = r"8 6 4 4 1";
        assert_eq!(part2(input), 1);

        let input = r"1 3 6 7 9";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn combined() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), 4);
    }
}
