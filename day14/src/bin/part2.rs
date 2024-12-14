use std::collections::HashSet;

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
    println!("{:?}", part1(input, 10000));
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

// And this function only gets compiled if the target OS is *not* linux

static COL_MOD: i32 = 101;
static ROW_MOD: i32 = 103;

fn part1(input: &str, n: u32) -> u32 {
    let mut players = get_players(input);

    for t in 1..n {
        // Movement.
        let mut tree_map = HashSet::new();
        for p in &mut players {
            p.pos.0 = (p.pos.0 + p.v.0).rem_euclid(ROW_MOD);
            p.pos.1 = (p.pos.1 + p.v.1).rem_euclid(COL_MOD);
            tree_map.insert(p.pos);
        }

        let mut run_count = 0;
        let mut stop = false;
        for row in 0..ROW_MOD {
            for col in 0..COL_MOD {
                if tree_map.contains(&(row, col)) {
                    run_count += 1;
                } else {
                    run_count = 0;
                }
                if run_count == 10 {
                    stop = true;
                }
            }
        }

        if stop {
            for row in 0..ROW_MOD {
                for col in 0..COL_MOD {
                    if tree_map.contains(&(row, col)) {
                        print!("X");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("after turn {}", t);
            return t;
        }
    }
    0
}
