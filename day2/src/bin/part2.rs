use core::cell::LazyCell;

use nom::bytes::complete::{is_a, tag};
use nom::character::complete::digit1;

use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

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
        // println!("level {} last {}", level, last);
        if *level <= last {
            is_ascending = false;
        }
        last = *level;
    }
    is_ascending
}

fn is_ascending_damped(report: &Vec<u32>) -> bool {
    if is_ascending_undamped(report) {
        true
    } else {
        for index in 0..report.len() {
            let mut adjusted_report = report.clone();
            adjusted_report.remove(index);
            if is_ascending_undamped(&adjusted_report) {
                return true;
            }
        }

        // No single deletion  helped.
        false
    }
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
fn is_descending_damped(report: &Vec<u32>) -> bool {
    if is_descending_undamped(report) {
        true
    } else {
        for index in 0..report.len() {
            dbg!(index);
            let mut adjusted_report = report.clone();
            adjusted_report.remove(index);
            dbg!(&adjusted_report);
            if is_descending_undamped(&adjusted_report) {
                return true;
            }
        }

        // No single deletion  helped.
        false
    }
}

fn big_gaps_undamped(report: &Vec<u32>) -> bool {
    dbg!(&report);
    let mut adjusted_report = report.clone();
    let mut big_gaps = false;
    // let mut iter = report.iter();
    // let mut prev: u32 = *iter.next().expect("Must have at least one level");
    let mut prev = adjusted_report.remove(0);
    for level in adjusted_report {
        println!("prev {prev} level {level} big {}", prev.abs_diff(level));
        if prev.abs_diff(level) > 3 {
            big_gaps = true;
        }
        prev = level;
    }
    big_gaps
}

fn big_gaps_damped(report: &Vec<u32>) -> bool {
    if !big_gaps_undamped(report) {
        false
    } else {
        for index in 0..report.len() {
            let mut adjusted_report = report.clone();
            adjusted_report.remove(index);
            if !big_gaps_undamped(&adjusted_report) {
                return false;
            }
        }
        // No single deletion  helped.
        true
    }
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            // filters only good reports.
            parse_report(line).ok()
        })
        .filter(|(_remain, report)| {
            let is_ascending = is_ascending_damped(report);
            let is_descending = is_descending_damped(report);
            let big_gaps = big_gaps_damped(report);

            !big_gaps && (is_ascending || is_descending)
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
