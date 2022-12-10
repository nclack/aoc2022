use std::ops::Index;

use itertools::Itertools;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::{map, opt},
    multi::many1_count,
    sequence::terminated,
    IResult,
};

struct Image<'a> {
    data: &'a [u8],
    shape: (usize, usize),
    stride: usize,
}

impl<'a> Index<(usize, usize)> for Image<'a> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.stride]
    }
}

fn parse(input: &str) -> IResult<&str, Image> {
    let (rest, width) = terminated(map(digit1, |d: &str| d.len()), line_ending)(input)?;
    let (rest, height) = many1_count(terminated(digit1, opt(line_ending)))(rest)?;
    Ok((
        rest,
        Image {
            data: input.as_bytes(),
            shape: (width, (height + 1)),
            stride: width + 1,
        },
    ))
}

fn scan<IX, IY>(im: &Image, dr: (isize, isize), xs: IX, ys: IY) -> Vec<u8>
where
    IX: IntoIterator<Item = usize> + Clone,
    IY: IntoIterator<Item = usize>,
{
    let mut mx = im.data.to_vec();
    let w = im.shape.0 as usize;
    for y in ys {
        for x in xs.clone() {
            mx[x + y * (w + 1)] = [dr, (0, 0)]
                .into_iter()
                .map(|dr| ((x as isize + dr.0) as usize, (y as isize + dr.1) as usize))
                .map(|r| mx[r.0 + r.1 * (w + 1)])
                .max()
                .unwrap()
        }
    }
    mx
}

fn scan2<IX, IY>(im: &Image, dr: (isize, isize), xs: IX, ys: IY) -> Vec<usize>
where
    IX: IntoIterator<Item = usize> + Clone,
    IY: IntoIterator<Item = usize>,
{
    let mut out: Vec<_> = (0..im.stride * im.shape.1).collect();
    for y in ys {
        for x in xs.clone() {
            let neighbor =
                ((x as isize + dr.0) + (y as isize + dr.1) * im.stride as isize) as usize;
            let cur = x + y * im.stride;
            // set the current cell to the location of the highest tree seen
            // along the direction set by dr
            out[cur] = if im.data[out[neighbor]] >= im.data[cur] {
                neighbor
            } else {
                cur
            };
        }
    }
    out
}

/*
30373
25512
65332
33549
35390

top
012345
67890
*/

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, im) = parse(input).unwrap();

    let tmp=scan2(&im, (0, -1), 0..im.shape.0, 1..im.shape.1);
    p2(&tmp,im.shape.0,im.shape.1);

    let top = scan(&im, (0, -1), 0..im.shape.0, 1..im.shape.1);
    let left = scan(&im, (-1, 0), 1..im.shape.0, 0..im.shape.1);
    let bot = scan(&im, (0, 1), 0..im.shape.0, (0..im.shape.1 - 1).rev());
    let right = scan(&im, (1, 0), (0..im.shape.0 - 1).rev(), 0..im.shape.1);

    let mut count = 0;
    for y in 0..im.shape.1 {
        for x in 0..im.shape.0 {
            let h = im[(x, y)] as i8;

            let t = if y > 0 {
                top[x + (y - 1) * im.stride] as i8
            } else {
                -1
            };
            let l = if x > 0 {
                left[x - 1 + y * im.stride] as i8
            } else {
                -1
            };
            let b = if y + 1 < im.shape.1 {
                bot[x + (y + 1) * im.stride] as i8
            } else {
                -1
            };
            let r = if x + 1 < im.shape.1 {
                right[x + 1 + y * im.stride] as i8
            } else {
                -1
            };

            count += (t < h || l < h || b < h || r < h) as usize;
        }
    }
    count
}

fn p(data: &[u8], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            print!("{}", data[x + y * (w + 1)] - b'0');
        }
        println!("");
    }
    println!("");
}


fn p2(data: &[usize], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            print!("{:3}", data[x + y * (w + 1)]);
        }
        println!("");
    }
    println!("");
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, im) = parse(input).unwrap();

    let top = scan(&im, (0, -1), 0..im.shape.0, 1..im.shape.1);
    let left = scan(&im, (-1, 0), 1..im.shape.0, 0..im.shape.1);
    let bot = scan(&im, (0, 1), 0..im.shape.0, (0..im.shape.1 - 1).rev());
    let right = scan(&im, (1, 0), (0..im.shape.0 - 1).rev(), 0..im.shape.1);

    p(&top, im.shape.0, im.shape.1);
    p(&left, im.shape.0, im.shape.1);
    p(&bot, im.shape.0, im.shape.1);
    p(&right, im.shape.0, im.shape.1);

    todo!()
}

#[test]
fn day8() {
    assert_eq!(21, part1(include_str!("../assets/day8.test.txt")));
    assert_eq!(8, part2(include_str!("../assets/day8.test.txt")));
}
