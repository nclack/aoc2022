use nom::{
    character::complete::{alpha1, line_ending},
    combinator::{map, opt},
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult,
};

fn line(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, opt(line_ending))(input)
}

fn items(items: &[u8]) -> u64 {
    items
        .into_iter()
        .map(|&c| {
            if b'a' <= c && c <= b'z' {
                c - b'a' + 1
            } else if b'A' <= c && c <= b'Z' {
                c - b'A' + 27
            } else {
                panic!("unexpected char {}", c)
            }
        })
        .fold(0u64, |obs, priority| obs | (1 << priority))
}

pub(crate) fn part1(input: &str) -> u32 {
    let (_, x) = fold_many1(
        map(line, |backpack: &str| {
            let b = backpack.as_bytes();
            let (l, r) = b.split_at(b.len() / 2);
            // priority of the shared item
            (items(l) & items(r)).trailing_zeros()
        }),
        || 0,
        |acc, p| acc + p,
    )(input)
    .unwrap();
    x
}

pub(crate) fn part2(input: &str) -> u32 {
    let (_, ps) = many1(map(line, |backpack: &str| items(backpack.as_bytes())))(input).unwrap();
    ps.chunks_exact(3)
        .flat_map(|c| c.iter().map(|&e| e).reduce(|acc, p| acc & p))
        .map(|p| p.trailing_zeros())
        .sum()
}

#[test]
fn day3() {
    assert_eq!(157, part1(include_str!("../assets/day3.test.txt")));
    assert_eq!(70, part2(include_str!("../assets/day3.test.txt")));
}
