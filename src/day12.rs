use std::{cmp::Reverse, collections::BinaryHeap, iter::repeat};

use nom::{
    character::complete::{alpha1, line_ending},
    combinator::{map, opt},
    multi::many0_count,
    sequence::terminated,
    IResult,
};

struct Image {
    data: Vec<u8>,
    shape: (usize, usize),
    stride: usize,
    start: isize,
    end: isize,
}

impl Image {
    fn nelem(&self) -> usize {
        self.shape.1 * self.stride
    }

    fn as_pos(&self, i: isize) -> (isize, isize) {
        (i % self.stride as isize, i / self.stride as isize)
    }

    fn in_bounds(&self, i: isize) -> bool {
        let (x, y) = self.as_pos(i);
        0 <= x && x < self.shape.0 as isize && 0 <= y && y < self.shape.1 as isize
    }
}

fn parse(input: &str) -> IResult<&str, Image> {
    let (rest, width) = terminated(map(alpha1, |d: &str| d.len()), opt(line_ending))(input)?;
    let (rest, height) = many0_count(terminated(alpha1, opt(line_ending)))(rest)?;
    let mut data = input.as_bytes().to_owned();
    let mut start = 0;
    let mut end = 0;
    for (i, d) in data.iter_mut().enumerate() {
        match *d {
            b'S' => {
                start = i as isize;
                *d = b'a';
            }
            b'E' => {
                end = i as isize;
                *d = b'z';
            }
            _ => {}
        }
    }
    Ok((
        rest,
        Image {
            data,
            shape: (width, (height + 1)),
            stride: width + 1,
            start,
            end,
        },
    ))
}

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, im) = parse(input).unwrap();
    let mut steps: Vec<usize> = vec![1 << 30; im.nelem()]; // set to something big

    // newline's (10) are much less than b'a' so paths can go down there, but
    // they'll never come back up

    steps[im.start as usize] = 0;
    let mut q: BinaryHeap<_> = repeat((Reverse(0), im.start as usize)).take(1).collect();
    let deltas = [-1, 1, -(im.stride as isize), im.stride as isize];
    while let Some((_, cur)) = q.pop() {
        if cur == im.end as usize {
            return steps[cur];
        }

        for n in deltas
            .iter()
            .map(|d| cur as isize + d)
            .filter(|&n| im.in_bounds(n))
            .map(|n| n as usize)
            .filter(|&n| im.data[n] <= im.data[cur] + 1)
        {
            if steps[cur] + 1 < steps[n] {
                steps[n] = steps[cur] + 1;
                q.push((Reverse(steps[n]), n));
            }
        }
    }
    panic!("Couldn't reach destination!");
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, im) = parse(input).unwrap();
    let mut steps: Vec<usize> = vec![1 << 30; im.nelem()];
    let mut mn = 1 << 30;

    steps[im.end as usize] = 0;
    let mut q: BinaryHeap<_> = repeat((Reverse(0), im.end as usize)).take(1).collect();
    let deltas = [-1, 1, -(im.stride as isize), im.stride as isize];
    while let Some((_, cur)) = q.pop() {
        if im.data[cur] == b'a' {
            mn = mn.min(steps[cur]);
        }

        for n in deltas
            .iter()
            .map(|d| cur as isize + d)
            .filter(|&n| im.in_bounds(n))
            .map(|n| n as usize)
            .filter(|&n| im.data[n] + 1 >= im.data[cur])
        {
            if steps[cur] + 1 < steps[n] {
                steps[n] = steps[cur] + 1;
                q.push((Reverse(steps[n]), n));
            }
        }
    }
    mn
}

#[test]
fn day12() {
    assert_eq!(31, part1(include_str!("../assets/day12.test.txt")));
    assert_eq!(29, part2(include_str!("../assets/day12.test.txt")));
}
