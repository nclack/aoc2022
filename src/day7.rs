use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, not_line_ending},
    combinator::{map, map_res, opt, value},
    multi::{fold_many1, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Command<'a> {
    Root,
    Pop,
    Push(&'a str),
    Size(usize),
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    fn ls(input: &str) -> IResult<&str, usize> {
        preceded(
            terminated(tag("$ ls"), line_ending),
            fold_many1(
                terminated(
                    alt((
                        value(0, tuple((tag("dir "), alpha1))),
                        map_res(terminated(digit1, not_line_ending), |sz| {
                            usize::from_str_radix(sz, 10)
                        }),
                    )),
                    opt(line_ending),
                ),
                || 0,
                |acc, sz| acc + sz,
            ),
        )(input)
    }

    many1(terminated(
        alt((
            value(Command::Root, tag("$ cd /")),
            value(Command::Pop, tag("$ cd ..")),
            map(preceded(tag("$ cd "), alpha1), |dir| Command::Push(dir)),
            map(ls, |sz| Command::Size(sz)),
        )),
        opt(line_ending),
    ))(input)
}

fn build_dirs(commands: Vec<Command>) -> HashMap<String, usize> {
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut cwd = Vec::new();
    for cmd in commands {
        match cmd {
            Command::Root => cwd.clear(),
            Command::Pop => {
                cwd.pop();
            }
            Command::Push(d) => {
                cwd.push(d);
            }
            Command::Size(sz) => {
                let mut path = String::new();
                *dirs.entry("/".to_string()).or_default() += sz;
                for d in cwd.iter() {
                    path += d;
                    path += "/";
                    *dirs.entry(path.clone()).or_default() += sz;
                }
            }
        }
    }
    dirs
}

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, commands) = parse(input).unwrap();
    let dirs = build_dirs(commands);
    dirs.values().filter(|&sz| *sz <= 100000).sum()
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, commands) = parse(input).unwrap();
    let dirs = build_dirs(commands);

    let remaining = 70000000 - dirs.get("/").unwrap();
    let thresh = 30000000 - remaining;

    *dirs.values().filter(|&sz| *sz > thresh).min().unwrap()
}

#[test]
fn day7() {
    assert_eq!(95437, part1(include_str!("../assets/day7.test.txt")));
    assert_eq!(24933642, part2(include_str!("../assets/day7.test.txt")));
}
