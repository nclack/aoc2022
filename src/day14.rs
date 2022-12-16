use std::{
    collections::HashSet,
    iter::repeat,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Paths = Vec<Vec<(i16, i16)>>;
struct Image {
    data: HashSet<(i16, i16)>,
    floor: i16,
}

fn parse(input: &str) -> IResult<&str, Paths> {
    fn number(input: &str) -> IResult<&str, i16> {
        map_res(digit1, |d| i16::from_str_radix(d, 10))(input)
    }
    let point = separated_pair(number, char(','), number);
    let path = separated_list1(tag(" -> "), point);
    separated_list1(line_ending, path)(input)
}

fn draw(paths: Paths) -> Image {
    fn sort(a: i16, b: i16) -> (i16, i16) {
        (a.min(b), a.max(b))
    }
    let mut im = HashSet::new();
    let mut floor = 0;
    for path in paths {
        for (&(x0, y0), &(x1, y1)) in path[..path.len() - 1].iter().zip(&path[1..]) {
            let (x0, x1) = sort(x0, x1);
            let (y0, y1) = sort(y0, y1);
            for (x, y) in (x0..=x1).cartesian_product(y0..=y1) {
                im.insert((x, y));
                floor = floor.max(y);
            }
        }
    }
    Image { data: im, floor }
}

fn add(a: (i16, i16), b: (i16, i16)) -> (i16, i16) {
    (a.0 + b.0, a.1 + b.1)
}

pub(crate) fn part1(input: &str) -> usize {
    fn drop(im: &mut Image) -> bool {
        let mut pos = (500, 0);
        let moves = [(0, 1), (-1, 1), (1, 1)];
        'step: while pos.1 <= im.floor {
            for m in moves {
                let new = add(pos, m);
                if !im.data.contains(&new) {
                    pos = new;
                    continue 'step;
                }
            }
            im.data.insert(pos);
            return true;
        }
        false
    }

    let (_rest, paths) = parse(input).unwrap();
    let mut im = draw(paths);
    repeat(1).take_while(|_| drop(&mut im)).sum()
}

pub(crate) fn part2(input: &str) -> usize {
    fn drop(im: &mut Image) -> bool {
        let mut pos = (500, 0);
        let moves = [(0, 1), (-1, 1), (1, 1)];
        'step: while !im.data.contains(&(500,0)) {
            for m in moves {
                let new = add(pos, m);
                if new.1<im.floor+2 && !im.data.contains(&new) {
                    pos = new;
                    continue 'step;
                }
            }
            im.data.insert(pos);
            return true;
        }
        false
    }

    let (_rest, paths) = parse(input).unwrap();
    let mut im = draw(paths);
    repeat(1).take_while(|_| drop(&mut im)).sum()
}

#[test]
fn day14() {
    assert_eq!(24, part1(include_str!("../assets/day14.test.txt")));
    assert_eq!(93, part2(include_str!("../assets/day14.test.txt")));
}