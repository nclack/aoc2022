use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, line_ending, space0},
    combinator::{map, map_res, opt, value},
    multi::{many1, many1_count},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let item = map(delimited(char('['), alpha1, char(']')), |s: &str| {
        s.chars().next()
    });
    terminated(
        many1(terminated(
            alt((item, value(None, tag("   ")))),
            opt(char(' ')),
        )),
        opt(line_ending),
    )(input)
}

fn stack_indexes(input: &str) -> IResult<&str, usize> {
    terminated(
        many1_count(delimited(space0, digit1, space0)),
        opt(line_ending),
    )(input)
}

fn table(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    terminated(many1(row), stack_indexes)(input)
}

#[derive(Debug)]
struct Move {
    count: usize,
    src: usize,
    dst: usize,
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(terminated(
        map(
            tuple((
                preceded(
                    tag("move "),
                    map_res(digit1, |e| usize::from_str_radix(e, 10)),
                ),
                preceded(
                    tag(" from "),
                    map_res(digit1, |e| usize::from_str_radix(e, 10)),
                ),
                preceded(
                    tag(" to "),
                    map_res(digit1, |e| usize::from_str_radix(e, 10)),
                ),
            )),
            |(count, src, dst)| Move { count, src, dst },
        ),
        opt(line_ending),
    ))(input)
}

#[derive(Debug)]
struct Document {
    table: Vec<Vec<Option<char>>>,
    moves: Vec<Move>,
}

fn document(input: &str) -> IResult<&str, Document> {
    map(
        separated_pair(table, char('\n'), moves),
        |(table, moves)| Document { table, moves },
    )(input)
}

fn readout(state: Vec<Vec<char>>) -> String {
    let ans: String = state
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect();
    ans
}

fn build_state(doc: &Document) -> Vec<Vec<char>> {
    let nstacks = doc.table[0].len(); // they'll all be the same length
    let mut state: Vec<Vec<char>> = vec![Vec::new(); nstacks];
    for row in doc.table.iter().rev() {
        for (i, c) in row.iter().enumerate() {
            if let Some(c) = c {
                state[i].push(*c);
            }
        }
    }
    state
}

pub(crate) fn part1(input: &str) -> String {
    let (_rest, doc) = document(input).unwrap();

    let mut state = build_state(&doc);
    for m in doc.moves {
        for _ in 0..m.count {
            let v = state[m.src - 1].pop().unwrap();
            state[m.dst - 1].push(v);
        }
    }

    readout(state)
}

pub(crate) fn part2(input: &str) -> String {
    let (_rest, doc) = document(input).unwrap();

    let mut state = build_state(&doc);
    for m in doc.moves {
        let n=state[m.src-1].len();
        let mut crates=state[m.src-1].split_off(n-m.count);
        state[m.dst-1].append(&mut crates);
    }

    readout(state)
}

#[test]
fn test_parse_row() {
    let (_rest, res) = row("[M]                     [N] [Z]    ").unwrap();
    assert_eq!(
        &res[..],
        &[
            Some('M'),
            None,
            None,
            None,
            None,
            None,
            Some('N'),
            Some('Z'),
            None
        ]
    );
}

#[test]
fn test_parse_stack_indexes() {
    let (_rest, n) = stack_indexes(" 1   2   3   4   5   6   7   8   9 ").unwrap();
    assert_eq!(n, 9);
}

#[test]
fn day5() {
    assert_eq!("CMZ", part1(include_str!("../assets/day5.test.txt")));
    assert_eq!("MCD", part2(include_str!("../assets/day5.test.txt")));
}
