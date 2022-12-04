use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::{map, opt},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

struct Interval(u8, u8);

fn parse(input: &str) -> IResult<&str, Vec<(Interval, Interval)>> {
    fn interval(input: &str) -> IResult<&str, Interval> {
        map(
            separated_pair(digit1::<&str, _>, char('-'), digit1),
            |(a, b)| Interval(a.parse().unwrap(), b.parse().unwrap()),
        )(input)
    }
    many1(terminated(
        separated_pair(interval, char(','), interval),
        opt(line_ending),
    ))(input)
}

pub(crate) fn part1(input: &str) -> usize {
    parse(input)
        .unwrap()
        .1
        .into_iter()
        .filter(|(a, b)| (a.0 <= b.0 && b.1 <= a.1) || (b.0 <= a.0 && a.1 <= b.1))
        .count() 
}

pub(crate) fn part2(input: &str) -> usize {
    parse(input)
        .unwrap()
        .1
        .into_iter()
        .filter(|(a, b)| a.0 <= b.1 && b.0 <= a.1)
        .count() 
}

#[test]
fn day4() {
    assert_eq!(2, part1(include_str!("../assets/day4.test.txt")));
    assert_eq!(4, part2(include_str!("../assets/day4.test.txt")));
}
