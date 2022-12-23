use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

type Pos = (i32, i32);

#[derive(Clone, Debug)]
struct Interval(i32, i32);

trait MinMax {
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
}

impl MinMax for Pos {
    fn min(&self, other: &Pos) -> Pos {
        (self.0.min(other.0), self.1.min(other.0))
    }
    fn max(&self, other: &Pos) -> Pos {
        (self.0.max(other.0), self.1.max(other.0))
    }
}

impl Interval {
    // /// Remove other from self. Treat intervals as [a,b).
    // fn remove(&self, other: &Interval) -> Vec<Interval> {
    //     match (self.0, self.1, other.0, other.1) {
    //         (a, _, _, d) if d <= a => vec![self.clone()], // other is too far left
    //         (_, b, c, _) if b <= c => vec![self.clone()], // other is too far right
    //         (a, b, c, d) if a >= c && b <= d => vec![],   // other contains self
    //         (a, b, c, d) if a < c && b > d => vec![Interval(a, c), Interval(d, b)], // self contains other
    //         (a, b, c, d) if c <= a && d < b => vec![Interval(d, b)], // other overlaps on the left
    //         (a, b, c, d) if a < c && b <= d => vec![Interval(c, b)], // other overlaps on the right
    //         _ => unreachable!(),
    //     }
    // }

    // /// Returns None if the intervals didn't intersect,
    // /// otherwise Some(union of the two intervals)
    // fn union(&self, other: &Interval) -> Option<Interval> {
    //     match (self.0, self.1, other.0, other.1) {
    //         (a, b, c, d) if a >= d || c >= b => None,
    //         (a, b, c, d) => Some(Interval(a.max(c), b.min(d))),
    //     }
    // }

    fn len(&self) -> usize {
        (self.1 - self.0 - 1) as usize
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Pos, Pos)>> {
    fn number(input: &str) -> IResult<&str, i32> {
        map_res(recognize(tuple((opt(char('-')), digit1))), |d| {
            i32::from_str_radix(d, 10)
        })(input)
    }
    fn pos(input: &str) -> IResult<&str, Pos> {
        map(
            tuple((tag("x="), number, tag(", y="), number)),
            |(_, x, _, y)| (x, y),
        )(input)
    }
    let row = tuple((
        preceded(tag("Sensor at "), pos),
        preceded(tag(": closest beacon is at "), pos),
    ));
    separated_list1(line_ending, row)(input)
}

fn dist(x: &Pos, y: &Pos) -> i32 {
    (x.0.abs_diff(y.0) + x.1.abs_diff(y.1)) as _
}

enum Bound {
    L(i32),
    R(i32),
}

impl Bound {
    fn unbox(&self) -> i32 {
        match self {
            &Bound::L(v) => v,
            &Bound::R(v) => v,
        }
    }
}

fn interval_at((sensor, beacon): &(Pos, Pos), y: i32) -> Option<[Bound; 2]> {
    let d = dist(sensor, beacon);
    let dy = sensor.1.abs_diff(y) as i32;
    let dx = d - dy;
    if dx <= 0 {
        None
    } else {
        Some([Bound::L(sensor.0 - dx), Bound::R(sensor.0 + dx + 1)])
    }
}

fn line_coverage(doc: &[((i32, i32), (i32, i32))], y: i32) -> Vec<Interval> {
    doc.into_iter()
        .flat_map(|obs| interval_at(&obs, y))
        .flat_map(|ival| ival.into_iter())
        .sorted_by(|a, b| match a.unbox().cmp(&b.unbox()) {
            v @ Ordering::Less => v,
            v @ Ordering::Greater => v,
            Ordering::Equal => match (a, b) {
                (Bound::L(_), Bound::R(_)) => Ordering::Less,
                (Bound::R(_), Bound::L(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        })
        .batching(|it| {
            let mut count = 0;
            let mut start = None;
            while let Some(b) = it.next() {
                match b {
                    Bound::L(x) => {
                        if count == 0 {
                            start = Some(x);
                        }
                        count += 1;
                    }
                    Bound::R(y) => {
                        count -= 1;
                        if count == 0 {
                            return Some(Interval(start.unwrap(), y));
                        }
                    }
                }
            }
            None
        })
        .collect()
}

fn process_line(input: &str, y: i32) -> usize {
    let (_rest, doc) = parse(input).unwrap();
    line_coverage(&doc, y).into_iter().map(|x| x.len()).sum()
}

pub(crate) fn part1(input: &str) -> usize {
    process_line(input, 2000000)
}

fn process_square(input: &str, mx: i32) -> usize {
    // TODO: alternate approach.
    //       Properly do polygon union's in the (x+1,x-y) space.
    //       It should be easier bc all the lines are axis aligned.
    //       Have to keep track of interior holes.
    //       Then looks for holes (counter-oriented contours) that
    //       are inside the problem's bbox.
    //       Bonus points if I can find an excuse to try PGA

    let (_rest, doc) = parse(input).unwrap();
    (0..mx)
        .flat_map(|y| {
            line_coverage(&doc, y)
                .into_iter()
                .map(|x| Interval(x.0.max(0), x.1.min(mx + 1)))
                .filter_map(move |x| {
                    if x.len() != mx as usize {
                        Some(x.1 as usize * 4_000_000 + y as usize)
                    } else {
                        None
                    }
                })
        })
        .next()
        .unwrap()
}

pub(crate) fn part2(input: &str) -> usize {
    process_square(input, 4000000)
}

#[test]
fn day15() {
    assert_eq!(
        26,
        process_line(include_str!("../assets/day15.test.txt"), 10)
    );
    assert_eq!(
        56000011,
        process_square(include_str!("../assets/day15.test.txt"), 20)
    );
}
