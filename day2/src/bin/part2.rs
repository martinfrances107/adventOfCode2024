use nom::bytes::complete::tag;
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

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            // filters only good reports.
            parse_report(line).ok()
        })
        .filter(|(_remain, report)| {
            // println!("report {report:#?}");

            let master_report_len = report.len();

            let (descending, report) = is_decending_damping(report);

            // println!("descending {descending}");

            let ascending = if descending {
                // Cannpot be both ascending and descending
                false
            } else {
                let (ascending, report) = is_ascending_damping(&report);
                if master_report_len - report.len() > 1 {
                    println!("assending:  but too many levels removed.");
                    return false;
                }
                println!("ascending {descending}");
                ascending
            };

            let (big_gaps, _report) = has_big_gaps_damping(&report);
            if master_report_len - report.len() > 1 {
                println!("big_gaps but too many levels removed.");
                return false;
            }
            // println!("big_gaps final{big_gaps}");
            let n_removed = master_report_len - report.len();
            // println!("{} {} {}", big_gaps, is_ascending, is_descending);
            n_removed < 2 && !big_gaps && (ascending || descending)
        })
        .count()
}

// checks for an ascending sequence.
//
// report the first failing value.
fn find_when_ascension_stop(report: &[u32]) -> Option<usize> {
    let mut last = u32::MIN;
    for (pos, level) in report.iter().enumerate() {
        if *level <= last {
            return Some(pos);
        }
        last = *level
    }
    return None;
}

// Returns the shortened array
//
fn is_ascending_damping(report: &[u32]) -> (bool, Vec<u32>) {
    let mut removing_first_is_valid = false;
    let mut removing_second_is_valid = false;
    match find_when_ascension_stop(report) {
        Some(r_pos) => {
            // Remove first
            let mut report_missing_left_value = report.to_vec();
            let _removed = report_missing_left_value.remove(r_pos - 1);

            if let Some(_pos) = find_when_ascension_stop(&report_missing_left_value) {
                removing_first_is_valid = true;
            }

            let mut report_missing_right_value = report.to_vec();
            let _removed = report_missing_right_value.remove(r_pos);
            if let Some(_pos) = find_when_ascension_stop(&report_missing_right_value) {
                removing_second_is_valid = true;
            }

            println!("descending_dmaping: first {removing_first_is_valid} second {removing_second_is_valid}");
            let conflict = removing_first_is_valid == true && removing_second_is_valid == true;
            assert!(!conflict);

            // Move the working version forward.
            if removing_first_is_valid {
                (true, report_missing_left_value)
            } else {
                (true, report_missing_right_value)
            }
        }
        None => (true, report.to_vec()),
    }
}

// checks for an decending sequence.
//
// report the first failing value.
fn find_when_descending_stops(report: &[u32]) -> Option<usize> {
    let mut last = u32::MAX;
    for (pos, level) in report.iter().enumerate() {
        if *level >= last {
            return Some(pos);
        }
        last = *level
    }
    return None;
}

fn is_decending_damping(report: &[u32]) -> (bool, Vec<u32>) {
    let mut removing_first_is_valid = false;
    let mut removing_second_is_valid = false;
    // println!("is_descending_damping {report:?}");
    match find_when_descending_stops(report) {
        Some(pos) => {
            let mut report_missing_left_value = report.to_vec();
            // println!("is_descending: position for removal {pos:?}");
            let removed = report_missing_left_value.remove(pos - 1);
            // println!("left removed {removed:?}");

            if let Some(_pos) = find_when_descending_stops(&report_missing_left_value) {
                removing_first_is_valid = true;
            }

            let mut report_missing_right_value = report.to_vec();

            let removed = report_missing_right_value.remove(pos);
            // println!("right removed {removed:?}");
            if let Some(_pos) = find_when_descending_stops(&report_missing_right_value) {
                removing_second_is_valid = true;
            }

            // println!("descending_dmaping: first {removing_first_is_valid} second {removing_second_is_valid}");
            let conflict = removing_first_is_valid == true && removing_second_is_valid == true;

            // Move the working version forward.
            // println!("is_descending_damping report {:?}", report);
            // println!(
            //     "is_descending_damping left report {:?}",
            //     report_missing_left_value
            // );
            // println!(
            //     "is_descending_damping right report {:?}",
            //     report_missing_right_value
            // );
            // println!("is_descending_damping removing left is value {removing_first_is_valid}, removing right is valid {removing_second_is_valid}");
            // assert!(!conflict);
            if removing_first_is_valid {
                (true, report_missing_left_value)
            } else {
                (true, report_missing_right_value)
            }
        }
        None => (true, report.to_vec()),
    }
}

fn find_big_gaps(report: &Vec<u32>) -> Option<usize> {
    println!("find_big_gaps() {:#?}", report);
    let mut iter = report.iter();
    let mut prev = *iter.next().expect("must have at least one element");
    //Subtle:  first (prev level) comparison is equivalent to a comparison between (x,x)
    report.iter().position(|level| {
        let big_gaps = prev.abs_diff(*level) > 3;
        if big_gaps {
            println!("find big_gaps found {prev} {level}");
        }
        prev = *level;
        big_gaps
    })
}

fn has_big_gaps_damping(report: &Vec<u32>) -> (bool, Vec<u32>) {
    let mut removing_first_is_valid = true;
    let mut removing_second_is_valid = true;

    let mut report_missing_left_value = report.clone();
    let mut report_missing_right_value = report.clone();

    match find_big_gaps(report) {
        Some(pos) => {
            // let mut report_missing_left_value = reports.clone();``
            let removed = report_missing_left_value.remove(pos - 1);
            println!("removed {removed}");

            if let Some(_pos) = find_big_gaps(&report_missing_left_value) {
                removing_first_is_valid = false;
            }

            // let mut report_missing_right_value = reports.clone();
            let _removed = report_missing_right_value.remove(pos);
            if let Some(_pos) = find_big_gaps(&report_missing_right_value) {
                removing_second_is_valid = false;
            }

            // println!("first {removing_first_is_valid} second {removing_second_is_valid}");
            // let conflict = removing_first_is_valid == true && removing_second_is_valid == true;
            // assert!(!conflict);

            // Move the working version forward.
            if removing_first_is_valid {
                (false, report_missing_left_value)
            } else if removing_second_is_valid {
                (false, report_missing_right_value)
            } else {
                (true, report.clone())
            }
        }

        None => (false, report.clone()),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example2() {
        // let input = r"7 6 4 2 1";
        // assert_eq!(part2(input), 1);

        // let input = r"1 2 7 8 9";
        // assert_eq!(part2(input), 0);

        // let input = r"9 7 6 2 1";
        // assert_eq!(part2(input), 0);

        let input = r"1 3 2 4 5";
        assert_eq!(part2(input), 1);

        // let input = r"8 6 4 4 1";
        // assert_eq!(part2(input), 1);
    }

    #[test]
    fn unit1() {
        let report: [u32; 5] = [1, 3, 2, 4, 5];
        assert_eq!(find_when_ascension_stop(&report), Some(2));

        let report = [1, 2, 3, 4, 5];
        assert_eq!(find_when_ascension_stop(&report), None);

        let report = [4, 3, 2, 1, 0];
        assert_eq!(find_when_ascension_stop(&report), Some(1));
    }

    #[test]
    fn unit2() {
        let report: [u32; 5] = [1, 3, 2, 4, 5];
        assert_eq!(find_when_descending_stops(&report), Some(1));

        let report = [1, 2, 3, 4, 5];
        assert_eq!(find_when_descending_stops(&report), Some(1));

        let report = [4, 3, 2, 1, 0];
        assert_eq!(find_when_descending_stops(&report), None);
    }
}
