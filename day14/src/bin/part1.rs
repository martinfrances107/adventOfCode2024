use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::combinator::{map, map_res, opt, recognize};
use nom::sequence::preceded;
use nom::{sequence::tuple, IResult};

#[derive(Debug, Eq, PartialEq)]
struct Player {
    pos: (i32, i32),
    v: (i32, i32),
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input, 100));
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        str::parse(s)
    })(input)?;

    Ok((i, number))
}

fn parse_position(input: &str) -> IResult<&str, (i32, i32)> {
    map(
        preceded(tag("p="), tuple((parse_i32, tag(","), parse_i32))),
        |(col, __, row)| (row, col),
    )(input)
}

fn parse_velocity(input: &str) -> IResult<&str, (i32, i32)> {
    map(
        preceded(tag("v="), tuple((parse_i32, tag(","), parse_i32))),
        |(col, __, row)| (row, col),
    )(input)
}
fn parse_player(input: &str) -> IResult<&str, Player> {
    map(
        tuple((parse_position, tag(" "), parse_velocity)),
        |(pos, _, v)| Player { pos, v },
    )(input)
}

fn get_players(input: &str) -> Vec<Player> {
    input
        .lines()
        .map(|line| {
            let (_, player) = parse_player(line).expect("Invalid input");
            player
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
static COL_MOD: i32 = 11;
#[cfg(test)]
static ROW_MOD: i32 = 7;
#[cfg(test)]
static COL_MID: i32 = 5;
#[cfg(test)]
static ROW_MID: i32 = 3;

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(test))]
static COL_MOD: i32 = 101;
#[cfg(not(test))]
static ROW_MOD: i32 = 103;
#[cfg(not(test))]
static COL_MID: i32 = 50;
#[cfg(not(test))]
static ROW_MID: i32 = 51;

fn part1(input: &str, n: u8) -> u32 {
    let mut players = get_players(input);

    for t in 0..n {
        // Movement.
        for p in &mut players {
            p.pos.0 = (p.pos.0 + p.v.0).rem_euclid(ROW_MOD);
            p.pos.1 = (p.pos.1 + p.v.1).rem_euclid(COL_MOD);
        }
        // println!("before turn {t}");
        // for row in 0..ROW_MOD {
        //     for col in 0..COL_MOD {
        //         if p.pos.0 == row && p.pos.1 == col {
        //             print!("1");
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }
        // println!("t = {t}");
    }

    let mut bathroom = HashMap::<(i32, i32), u32>::default();
    for p in players {
        match bathroom.get_mut(&p.pos) {
            Some(count) => {
                *count += 1;
            }
            None => {
                bathroom.insert(p.pos, 1);
            }
        }
    }

    println!();
    // Quad map
    for r in 0..ROW_MOD {
        for c in 0..COL_MOD {
            if r == ROW_MID || c == COL_MID {
                print!("XX");
            } else {
                match bathroom.get(&(r, c)) {
                    Some(value) => {
                        print!("{value} ")
                    }
                    None => {
                        print!(". ")
                    }
                }
            }
        }
        println!();
    }

    // Quadrant0 ( top,left )
    let mut quad0 = 0;
    for r in 0..ROW_MID {
        for c in 0..COL_MID {
            if let Some(n_players) = bathroom.get(&(r, c)) {
                quad0 += *n_players
            }
        }
    }
    // panic!();

    // Quadrant1 ( top, right )
    let mut quad1 = 0;
    for r in 0..ROW_MID {
        for c in COL_MID + 1..COL_MOD {
            if let Some(n_players) = bathroom.get(&(r, c)) {
                quad1 += *n_players
            }
        }
    }

    // Quadrant2 ( bottom, left )
    let mut quad2 = 0;
    for r in ROW_MID + 1..ROW_MOD {
        for c in 0..COL_MID {
            if let Some(n_players) = bathroom.get(&(r, c)) {
                quad2 += *n_players
            }
        }
    }

    // Quadrant3 ( bottom, right )
    let mut quad3 = 0;
    for r in ROW_MID + 1..ROW_MOD {
        for c in COL_MID + 1..COL_MOD {
            if let Some(n_players) = bathroom.get(&(r, c)) {
                quad3 += *n_players
            }
        }
    }
    dbg!(quad0);
    dbg!(quad1);
    dbg!(quad2);
    dbg!(quad3);

    quad0 * quad1 * quad2 * quad3
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn number() {
        let input = r"-2";
        assert_eq!(parse_i32(input), Ok(("", -2i32)));
        let input = r"-200";
        assert_eq!(parse_i32(input), Ok(("", -200i32)));
        let input = r"0";
        assert_eq!(parse_i32(input), Ok(("", 0i32)));
    }

    #[test]
    fn players() {
        let input = r"p=0,4 v=3,-3
p=6,3 v=-1,-3";

        let expected = vec![
            Player {
                pos: (0, 4),
                v: (3, -3),
            },
            Player {
                pos: (6, 3),
                v: (-1, -3),
            },
        ];

        assert_eq!(get_players(input), expected);
    }

    #[test]
    fn test_single_player() {
        let input = "p=2,4 v=2,-3";
        part1(input, 6);
    }

    #[test]
    fn one_hundred_turns() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part1(input, 100), 1);
    }
}
