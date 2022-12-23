use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let snacks = many1(map_res(terminated(digit1, opt(char('\n'))), |src| {
        u32::from_str_radix(src, 10)
    }));
    let mut elves = many1(terminated(snacks, opt(char('\n'))));
    elves(input)
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .unwrap()
        .1
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut elves: Vec<_> = parse(input)
        .unwrap()
        .1
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect();
    elves.sort();
    elves.iter().rev().take(3).sum()
}

#[test]
fn day1() {
    assert_eq!(24000, part1(include_str!("../assets/day1.test.txt")));
    assert_eq!(45000, part2(include_str!("../assets/day1.test.txt")));
}
