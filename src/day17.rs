use std::{
    cell::Cell,
    ops::{Index, IndexMut, Range},
};

use nom::{branch::alt, character::complete::char, combinator::value, multi::many1, IResult};

fn left(x: u8) -> u8 {
    x << 1
}

fn right(x: u8) -> u8 {
    x >> 1
}

fn check_left(x: &u8) -> bool {
    x & 0b0100_0000 == 0
}

fn check_right(x: &u8) -> bool {
    x & 0b0000_0001 == 0
}

fn parse(input: &str) -> IResult<&str, Vec<(fn(&u8) -> bool, fn(u8) -> u8)>> {
    many1(alt((
        value((check_left as _, left as _), char('<')),
        value((check_right as _, right as _), char('>')),
    )))(input)
}

struct Block {
    rows: Vec<u8>,
}

impl Block {
    fn new(i: usize) -> Block {
        #[rustfmt::skip]
        let rows=[
                //   L6543210R
                //    1000001
                 &[0b00011110][..], // bottom
                 &[0b00001000,
                   0b00011100,
                   0b00001000][..],
                 &[0b00011100,
                   0b00000100,
                   0b00000100][..],
                 &[0b00010000,
                   0b00010000,
                   0b00010000,
                   0b00010000][..],
                 &[0b00011000,
                   0b00011000][..]];
        Block {
            rows: rows[i].to_vec(),
        }
    }

    fn check(&self, op: fn(&u8) -> bool) -> bool {
        self.rows.iter().all(op)
    }

    fn apply(&mut self, op: fn(u8) -> u8) {
        for r in self.rows.iter_mut() {
            *r = op(*r);
        }
    }
}

struct CircBuf {
    store: Vec<u8>,
    len: usize,
    tmp: Cell<Vec<u8>>,
}

impl CircBuf {
    fn new(cap: usize) -> Self {
        Self {
            store: vec![0; cap],
            len: 0,
            tmp: Cell::new(Vec::new()),
        }
    }
    fn len(&self) -> usize {
        self.len
    }

    fn append(&mut self, other: &mut Vec<u8>) {
        // ^ call signature is just to match Vec::append()
        let n = self.store.len();
        for (i,v) in other.into_iter().enumerate() {
            self.store[(self.len+i)%n]=*v;
        }
        self.len+=other.len();
    }
}

impl Index<usize> for CircBuf {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.store[(index) % self.store.len()]
    }
}

impl IndexMut<usize> for CircBuf {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let n = self.store.len();
        &mut self.store[(index) % n]
    }
}

impl Index<Range<usize>> for CircBuf {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        let n = self.store.len();
        if range.start / n == range.end / n {
            &self.store[Range {
                start: range.start % n,
                end: range.end % n,
            }]
        } else {
            self.tmp
                .set([&self.store[range.start % n..], &self.store[..range.end % n]].concat());
            unsafe { &(*self.tmp.as_ptr())[..] }
        }
    }
}

fn solve(input: &str, nrocks:usize) -> usize {
    let (_rest, commands) = parse(input).unwrap();

    let mut ip = 0;
    let mut ground = CircBuf::new(100);

    for irock in 0..nrocks {

        if irock % 1000000 == 0 {
            println!("HERE {irock}");
        }

        let mut block = Block::new(irock % 5);
        let mut y = ground.len() + 3;

        'moving: loop {
            // move left/right
            let (wall, mv) = commands[ip];
            ip = (ip + 1) % commands.len();

            let no_collision = block.check(wall)
                && if y < ground.len() {
                    let w = block.rows.len().min(ground.len() - y);
                    ground[y..y + w]
                        .into_iter()
                        .zip(&block.rows[0..w])
                        .all(|(&a, &b)| {
                            // collision when bits are true in both, so "and".
                            a & mv(b) == 0
                        })
                } else {
                    true
                };

            if no_collision {
                block.apply(mv);
            }

            // move down
            if y == 0 {
                break 'moving;
            } else if y <= ground.len() {
                // sanity: when y==ground.len(), w should be 1. (ok)
                let w = block.rows.len().min(ground.len() - y + 1);
                // check   [y-1..ground.len()]
                // against [0..w]
                if ground[(y - 1)..(y - 1 + w)]
                    .into_iter()
                    .zip(&block.rows[0..w])
                    .any(|(&a, &b)| {
                        // collision when bits are true in both, so "and".
                        a & b > 0
                    })
                {
                    break 'moving;
                }
            }
            y -= 1;
        }

        // freeze
        let k = block.rows.len();
        if y + k >= ground.len() {
            // => k-(ground.len()-y)>=0
            let n = y + k - ground.len();
            ground.append(&mut vec![0; n]);
        }
        for i in 0..k {
            ground[y + i] |= block.rows[i];
        }
    }
    // print_ground(&ground);
    ground.len()
}

fn print_ground(ground: &Vec<u8>) {
    for (i, row) in ground.iter().enumerate().rev() {
        print!("{i:5} ");
        for b in (0..7).rev() {
            print!("{}", if (row >> b) & 1 == 1 { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

pub(crate) fn part1(input:&str)->usize {
    solve(input,2022)
}

pub(crate) fn part2(input:&str)->usize {
    solve(input,1000000000000)
}

#[test]
fn day17() {
    assert_eq!(3068, part1(include_str!("../assets/day17.test.txt")));
    assert_eq!(1514285714288, part2(include_str!("../assets/day17.test.txt")));
}
