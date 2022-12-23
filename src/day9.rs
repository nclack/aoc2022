use std::{collections::HashSet, iter::repeat};

use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending, space1},
    combinator::{map, map_res, opt, value},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
    U,
    D,
}

struct Command {
    dir: Direction,
    count: usize,
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    many1(terminated(
        map(
            separated_pair(
                alt((
                    value(Direction::L, char('L')),
                    value(Direction::R, char('R')),
                    value(Direction::U, char('U')),
                    value(Direction::D, char('D')),
                )),
                space1,
                map_res(digit1, |d| usize::from_str_radix(d, 10)),
            ),
            |(dir, count)| Command { dir, count },
        ),
        opt(line_ending),
    ))(input)
}

fn step(dir: Direction) -> [i16; 2] {
    match dir {
        Direction::L => [-1, 0],
        Direction::R => [1, 0],
        Direction::U => [0, 1],
        Direction::D => [0, -1],
    }
}

fn add(a: [i16; 2], b: [i16; 2]) -> [i16; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

fn step_tail(dr: [i16; 2], head: [i16; 2]) -> ([i16; 2], [i16; 2]) {
    let h = add(head, dr);
    match h {
        [x, y] if -1 <= x && x <= 1 && -1 <= y && y <= 1 => ([0, 0], h),
        [-2, -2] => ([-1, -1], [-1, -1]),
        [-2, 2] => ([-1, 1], [-1, 1]),
        [2, -2] => ([1, -1], [1, -1]),
        [2, 2] => ([1, 1], [1, 1]),
        [-2, y] => ([-1, y], [-1, 0]),
        [2, y] => ([1, y], [1, 0]),
        [x, -2] => ([x, -1], [0, -1]),
        [x, 2] => ([x, 1], [0, 1]),
        _ => unimplemented!(),
    }
}

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, commands) = parse(input).unwrap();

    commands
        .into_iter()
        .flat_map(|c| repeat(c.dir).take(c.count))
        .scan(([0, 0], [0, 0]), |(t, h), dir| {
            let (dt, newh) = step_tail(step(dir), *h);
            *t = add(*t, dt);
            *h = newh;
            Some(*t)
        })
        .collect::<HashSet<_>>()
        .len()
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, commands) = parse(input).unwrap();

    commands
        .into_iter()
        .flat_map(|c| repeat(c.dir).take(c.count))
        .scan((0, [[0, 0]; 10]), |(t, knots), dir| {
            let mut dr = step(dir);
            for i in 0..9 {
                (dr, knots[i]) = step_tail(dr, knots[i]);
            }
            knots[9] = add(knots[9], dr);
            *t += 1;
            Some(knots[9])
        })
        .collect::<HashSet<_>>()
        .len()
}

#[test]
fn day9() {
    assert_eq!(13, part1(include_str!("../assets/day9.test.txt")));
    assert_eq!(1, part2(include_str!("../assets/day9.test.txt")));
    assert_eq!(36, part2(include_str!("../assets/day9.test.2.txt")));
}
