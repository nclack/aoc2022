use std::iter::{once, repeat};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(terminated(
        alt((
            map(
                preceded(
                    tag("addx "),
                    map_res(recognize(tuple((opt(char('-')), digit1))), |d| {
                        i32::from_str_radix(d, 10)
                    }),
                ),
                |x| (2, x),
            ),
            map(tag("noop"), |_| (1, 0)),
        )),
        opt(line_ending),
    ))(input)
}

pub(crate) fn part1(input: &str) -> i32 {
    let (_rest, effects) = parse(input).unwrap();
    effects
        .into_iter()
        .flat_map(|(dclock, dx)| {
            repeat(0)
                .take(if dclock > 1 { dclock as usize - 1 } else { 0 })
                .chain(once(dx))
        })
        .enumerate()
        .scan(1, |x, (i, dx)| {
            let score = (*x) * (i + 1) as i32;
            *x += dx;
            Some(score)
        })
        .skip(19)
        .step_by(40)
        .sum()
}

pub(crate) fn part2(input: &str) {
    let (_rest, effects) = parse(input).unwrap();
    let im: Vec<_> = effects
        .into_iter()
        .flat_map(|(dclock, dx)| {
            repeat(0)
                .take(if dclock > 1 { dclock as usize - 1 } else { 0 })
                .chain(once(dx))
        })
        .enumerate()
        .scan(1, |x, (i, dx)| {
            let phase = (i % 40) as i32;
            let out = if *x - 1 <= phase && phase <= *x + 1 {
                '#'
            } else {
                ' '
            };
            *x += dx;
            Some(out)
        })
        .collect();

    for row in &im.into_iter().chunks(40) {
        println!("{}", String::from_iter(row));
    }
}

#[test]
fn day9() {
    assert_eq!(13140, part1(include_str!("../assets/day10.test.txt")));
    assert_eq!((), part2(include_str!("../assets/day10.test.txt")));
}
