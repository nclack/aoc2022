use std::{cmp::Reverse, collections::BinaryHeap, ops::Index};

use itertools::Itertools;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::{map, opt},
    multi::many0_count,
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
    let (rest, width) = terminated(map(digit1, |d: &str| d.len()), opt(line_ending))(input)?;
    let (rest, height) = many0_count(terminated(digit1, opt(line_ending)))(rest)?;
    Ok((
        rest,
        Image {
            data: input.as_bytes(),
            shape: (width, (height + 1)),
            stride: width + 1,
        },
    ))
}

// segmented max scan
fn scan<IX, IY>(im: &Image, dr: (isize, isize), xs: IX, ys: IY) -> Vec<usize>
where
    IX: IntoIterator<Item = usize> + Clone,
    IY: IntoIterator<Item = usize>,
{
    let mut out: Vec<_> = (0..im.stride * im.shape.1).collect();
    for y in ys {
        let ny = (y as isize + dr.1) as usize;
        for x in xs.clone() {
            let nx = (x as isize + dr.0) as usize;
            let neighbor = nx + ny * im.stride;
            let cur = x + y * im.stride;
            // set the current cell to the location of the highest tree seen
            // along the direction set by dr
            out[cur] = if im.data[cur] > im.data[out[neighbor]] {
                cur
            } else {
                out[neighbor]
            };
        }
    }
    out
}

pub(crate) fn part1(input: &str) -> usize {
    let (_rest, im) = parse(input).unwrap();

    let top = scan(&im, (0, -1), 0..im.shape.0, 1..im.shape.1);
    let left = scan(&im, (-1, 0), 1..im.shape.0, 0..im.shape.1);
    let bot = scan(&im, (0, 1), 0..im.shape.0, (0..im.shape.1 - 1).rev());
    let right = scan(&im, (1, 0), (0..im.shape.0 - 1).rev(), 0..im.shape.1);

    // if a cell has the highest tree then it's index will appear in one of
    // the scans at a corresponding cell

    (0..im.shape.0)
        .cartesian_product(0..im.shape.1)
        .map(|(x, y)| x + y * im.stride)
        .filter(|&i| top[i] == i || left[i] == i || bot[i] == i || right[i] == i)
        .count()
}

//
// Abandon all hope ye who enter here
//

#[derive(Clone)]
struct Interval {
    beg: usize,
    end: usize,
    h: u8,
    next: Option<usize>,
}

impl Interval {
    fn length(&self) -> usize {
        self.end - self.beg
    }
}

fn follow(intervals: &[Option<Interval>], pos: usize, h: u8) -> (usize, usize) {
    let mut pos = pos;
    let mut score = 1;
    while let Some(next) = {
        let ival=intervals[pos].as_ref().unwrap();
        if ival.h < h {
            score = ival.length()
        }
        ival.next
    }  {
        pos = next;
    }
    return (score, pos);
}

fn watershed_rows(im: &Image, scores: &mut [usize]) {
    for y in 0..im.shape.1 {
        let mut intervals: Vec<Option<Interval>> = vec![None; im.stride * im.shape.1];
        let mut queued = vec![false; im.stride * im.shape.1];
        let mut q = BinaryHeap::new();
        let row = &im.data[y * im.stride..(im.shape.0 + y * im.stride)];
        let scores_row = &mut scores[y * im.stride..(im.shape.0 + y * im.stride)];

        // Prep boundary
        intervals[0] = Some(Interval {
            beg: 0,
            end: 1,
            h: row[0],
            next: None,
        });
        intervals[im.shape.0 - 1] = Some(Interval {
            beg: im.shape.0 - 2,
            end: im.shape.0 - 1,
            h: row[im.shape.0 - 1],
            next: None,
        });
        scores_row[0] = 0;
        scores_row[im.shape.0 - 1] = 0;

        // init w values at min local minima
        {
            for (l, r) in (0..im.shape.0 - 2).zip(2..im.shape.0) {
                if row[l] >= row[l + 1] || row[l + 1] <= row[r] {
                    q.push(Reverse((row[l + 1], l + 1)));
                    queued[l + 1] = true;
                }
            }
        }

        // watershed
        while let Some(Reverse((h, i))) = q.pop() {
            let (left_score, beg) = {
                // i>0
                if intervals[i - 1].is_some() {
                    let (score, j) = follow(&intervals[..], i - 1, h);
                    let mut interval = intervals[j].as_mut().unwrap();
                    interval.next = Some(i);
                    (score, interval.beg)
                } else {
                    if i >= 1 && !queued[i - 1] {
                        q.push(Reverse((row[i - 1], i - 1)));
                        queued[i - 1] = true;
                    }
                    (1, i - 1)
                }
            };
            let (right_score, end) = {
                // i+1<im.shape.0
                if intervals[i + 1].is_some() {
                    let (score, j) = follow(&intervals[..], i + 1, h);
                    let mut interval = intervals[j].as_mut().unwrap();
                    interval.next = Some(i);
                    (score, interval.end)
                } else {
                    if i >= 1 && !queued[i + 1] {
                        q.push(Reverse((row[i + 1], i + 1)));
                        queued[i + 1] = true;
                    }
                    (1, i + 1)
                }
            };
            scores_row[i] *= left_score * right_score;
            intervals[i] = Some(Interval {
                beg,
                end,
                h,
                next: None,
            });
        }
    }
}

fn watershed_cols(im: &Image, scores: &mut [usize]) {
    for x in 0..im.shape.0 {
        let mut intervals: Vec<Option<Interval>> = vec![None; im.stride * im.shape.1];
        let mut queued = vec![false; im.stride * im.shape.1];
        let mut q = BinaryHeap::new();

        let col = |y: usize| im.data[x + y * im.stride];

        // Prep boundary
        intervals[0] = Some(Interval {
            beg: 0,
            end: 1,
            h: col(0),
            next: None,
        });
        intervals[im.shape.1 - 1] = Some(Interval {
            beg: im.shape.1 - 2,
            end: im.shape.1 - 1,
            h: col(im.shape.1 - 1),
            next: None,
        });
        scores[x] = 0;
        scores[x + im.stride * (im.shape.1 - 1)] = 0;

        // init w values at min local minima
        {
            for (l, r) in (0..im.shape.1 - 2).zip(2..im.shape.1) {
                if col(l) >= col(l + 1) || col(l + 1) <= col(r) {
                    q.push(Reverse((col(l + 1), l + 1)));
                    queued[l + 1] = true;
                }
            }
        }

        // watershed
        while let Some(Reverse((h, i))) = q.pop() {
            let (left_score, beg) = {
                // i>0
                if intervals[i - 1].is_some() {
                    let (score, j) = follow(&intervals[..], i - 1, h);
                    let mut interval = intervals[j].as_mut().unwrap();
                    interval.next = Some(i);
                    (score, interval.beg)
                } else {
                    if i >= 1 && !queued[i - 1] {
                        q.push(Reverse((col(i - 1), i - 1)));
                        queued[i - 1] = true;
                    }
                    (1, i - 1)
                }
            };
            let (right_score, end) = {
                // i+1<im.shape.1
                if intervals[i + 1].is_some() {
                    let (score, j) = follow(&intervals[..], i + 1, h);
                    let mut interval = intervals[j].as_mut().unwrap();
                    interval.next = Some(i);
                    (score, interval.end)
                } else {
                    if i >= 1 && !queued[i + 1] {
                        q.push(Reverse((col(i + 1), i + 1)));
                        queued[i + 1] = true;
                    }
                    (1, i + 1)
                }
            };
            scores[x + i * im.stride] *= left_score * right_score;
            intervals[i] = Some(Interval {
                beg,
                end,
                h,
                next: None,
            });
        }
    }
}

pub(crate) fn part2(input: &str) -> usize {
    let (_rest, im) = parse(input).unwrap();

    let mut scores = vec![1; im.stride * im.shape.1];
    watershed_rows(&im, &mut scores[..]);
    watershed_cols(&im, &mut scores[..]);

    scores.into_iter().max().unwrap()
}

#[test]
fn day8() {
    assert_eq!(21, part1(include_str!("../assets/day8.test.txt")));
    assert_eq!(8, part2(include_str!("../assets/day8.test.txt")));
}
