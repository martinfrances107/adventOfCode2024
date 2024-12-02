use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn parse_level(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_report(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(" "), parse_level)(input)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            // filters only good reports.
            parse_report(line).ok()
        })
        .filter(|(_remain, report)| {
            let mut last = u32::MIN;
            let mut is_ascending = true;
            for level in report {
                // println!("level {} last {}", level, last);
                if *level <= last {
                    is_ascending = false;
                }
                last = *level;
            }

            let mut last = u32::MAX;
            let mut is_descending = true;
            for level in report {
                if *level >= last {
                    is_descending = false;
                }
                last = *level;
            }

            let mut big_gaps = false;
            let mut iter = report.iter();
            let mut prev: u32 = *iter.next().expect("Must have at least one level");
            for level in iter {
                if prev.abs_diff(*level) > 3 {
                    big_gaps = true;
                }
                prev = *level;
            }

            // println!("{} {} {}", big_gaps, is_ascending, is_descending);
            !big_gaps && (is_ascending || is_descending)
        })
        .count()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"7 6 4 2 1";
        assert_eq!(part1(input), 1);

        let input = r"1 2 7 8 9";
        assert_eq!(part1(input), 0);

        let input = r"9 7 6 2 1";
        assert_eq!(part1(input), 0);

        let input = r"1 3 2 4 5";
        assert_eq!(part1(input), 0);

        let input = r"8 6 4 4 1";
        assert_eq!(part1(input), 0);
    }
}
