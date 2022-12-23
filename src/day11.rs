use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::{many0, many1, separated_list0},
    sequence::{preceded, terminated, tuple},
    IResult,
};

enum Op {
    Mul(isize),
    Add(isize),
    Square,
}

impl Op {
    fn apply(&self, x: isize) -> isize {
        match self {
            Op::Mul(y) => y * x,
            Op::Add(y) => y + x,
            Op::Square => x * x,
        }
    }
}

struct Monkey {
    items: VecDeque<isize>,
    op: Op,
    divisor: isize,
    next: [usize; 2],
    count: usize,
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(digit1, |d| isize::from_str_radix(d, 10))(input)
    }
    let monkey_title = tuple((tag("Monkey "), digit1, char(':'), line_ending));
    let starting_items = terminated(
        preceded(
            tag("  Starting items: "),
            separated_list0(tag(", "), number),
        ),
        line_ending,
    );
    let divisor = terminated(preceded(tag("  Test: divisible by "), number), line_ending);
    let on_true = terminated(
        preceded(tag("    If true: throw to monkey "), number),
        line_ending,
    );
    let on_false = terminated(
        preceded(tag("    If false: throw to monkey "), number),
        opt(line_ending),
    );
    let operation = terminated(
        preceded(
            tag("  Operation: new = old "),
            alt((
                map(tag("* old"), |_| Op::Square),
                map(preceded(tag("* "), number), |v| Op::Mul(v)),
                map(preceded(tag("+ "), number), |v| Op::Add(v)),
            )),
        ),
        line_ending,
    );
    many1(terminated(
        map(
            tuple((
                monkey_title,
                starting_items,
                operation,
                divisor,
                on_true,
                on_false,
            )),
            |(_, items, op, divisor, t, f)| Monkey {
                items: items.into(),
                op,
                divisor,
                next: [f as usize, t as usize],
                count: 0,
            },
        ),
        many0(line_ending),
    ))(input)
}

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, mut monkeys) = parse(input).unwrap();
    for _round in 0..20 {
        for m in 0..monkeys.len() {
            while let Some(worry) = monkeys[m].items.pop_front() {
                let worry = monkeys[m].op.apply(worry) / 3;
                let next = monkeys[m].next[((worry % monkeys[m].divisor) == 0) as usize];
                monkeys[next].items.push_back(worry);
                monkeys[m].count += 1;
            }
        }
    }
    monkeys
        .into_iter()
        .map(|m| m.count)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, mut monkeys) = parse(input).unwrap();

    let base: isize = monkeys.iter().map(|m| m.divisor).product();

    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            while let Some(worry) = monkeys[m].items.pop_front() {
                let worry = monkeys[m].op.apply(worry) % base;
                let next = monkeys[m].next[((worry % monkeys[m].divisor) == 0) as usize];
                monkeys[next].items.push_back(worry);
                monkeys[m].count += 1;
            }
        }
    }
    monkeys
        .into_iter()
        .map(|m| m.count)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

#[test]
fn day11() {
    assert_eq!(10605, part1(include_str!("../assets/day11.test.txt")));
    assert_eq!(2713310158, part2(include_str!("../assets/day11.test.txt")));
}
