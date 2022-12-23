use std::cmp::Ordering;

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Item {
    Num(u8),
    List(Vec<Item>),
}

struct Document {
    pairs: Vec<(Item, Item)>,
}

mod parse {
    use nom::{
        branch::alt,
        character::complete::{char, digit1, line_ending},
        combinator::{map, map_res, opt},
        multi::{many0, many1, separated_list0, separated_list1},
        sequence::{delimited, terminated, tuple},
        IResult,
    };

    use super::{Document, Item};

    fn number(input: &str) -> IResult<&str, u8> {
        map_res(digit1, |s| u8::from_str_radix(s, 10))(input)
    }

    fn list(input: &str) -> IResult<&str, Item> {
        map(
            delimited(
                char('['),
                separated_list0(char(','), alt((map(number, |x| Item::Num(x)), list))),
                char(']'),
            ),
            |items| Item::List(items),
        )(input)
    }

    pub(super) fn part1(input: &str) -> IResult<&str, Document> {
        map(
            many1(terminated(
                tuple((
                    terminated(list, line_ending),
                    terminated(list, opt(line_ending)),
                )),
                many0(line_ending),
            )),
            |pairs| Document { pairs },
        )(input)
    }

    pub(super) fn part2(input: &str) -> IResult<&str, Vec<Item>> {
        separated_list1(many1(line_ending), list)(input)
    }
}

fn cmp(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (&Item::Num(x), y @ Item::List(_)) => cmp(&Item::List(vec![Item::Num(x)]), y),
        (x @ Item::List(_), &Item::Num(y)) => cmp(x, &Item::List(vec![Item::Num(y)])),
        (Item::Num(x), Item::Num(y)) => {
            if x < y {
                Ordering::Less
            } else if x > y {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        (Item::List(x), Item::List(y)) => {
            for i in 0..x.len().min(y.len()) {
                let c = cmp(&x[i], &y[i]);
                if c != Ordering::Equal {
                    return c;
                }
            }
            cmp(&Item::Num(x.len() as u8), &Item::Num(y.len() as u8))
        }
    }
}

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, doc) = parse::part1(input).unwrap();
    doc.pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if cmp(&left, &right) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, doc) = parse::part2(input).unwrap();
    let dividers = [
        Item::List(vec![Item::List(vec![Item::Num(2)])]),
        Item::List(vec![Item::List(vec![Item::Num(6)])]),
    ];
    doc.iter()
        .chain(dividers[..].into_iter())
        .sorted_by(|&a, &b| cmp(a, b))
        .enumerate()
        .filter_map(|(i, e)| {
            if *e == dividers[0] || *e == dividers[1] {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>()
}

#[test]
fn day13() {
    assert_eq!(13, part1(include_str!("../assets/day13.test.txt")));
    assert_eq!(140, part2(include_str!("../assets/day13.test.txt")));
}
