

use nom::{branch::alt, character::complete::char, combinator::value, multi::many1, IResult};

const NBLOCKS: usize=5;

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
        let rows:[_;NBLOCKS]=[
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

fn depths(ground: &[u8])->[usize;7] {
    ground.into_iter().rev().enumerate().scan(
        (0,[0;7]), |(known,depths),(i,&v)| {
            for b in 0..7 {
                if (*known>>b)&1==0 && (v>>b)&1==1 {
                    *known |= 1<<b;
                    depths[b]=i;
                }
            }
            Some((*known,*depths))
        }
    ).filter(|&(known,_)| known==0b111_1111)
    .next().unwrap().1
}

fn solve(input: &str, nrocks:usize) -> usize {
    let (_rest, commands) = parse(input).unwrap();

    let period=commands.len()*NBLOCKS;
    
    let mut ground = Vec::new();
    let mut d=[0;7];
    let mut h=0;
    let mut maxd=0;

    simulate(&mut ground, nrocks, &commands);
    ground.len()

    // for irock in (0..nrocks).step_by(period) {
    //     simulate(&mut ground, period.min(nrocks-irock), &commands);
    //     d=depths(&ground);
    //     maxd=*d.iter().max().unwrap();
    //     print_ground(&ground,None);
    //     let dh=ground.len()-maxd;
    //     h+=dh;
    //     ground.drain(0..dh); // ground.len() - dh = maxd

    //     println!("{irock} {d:?} {dh} {h} {maxd}");
    // }
    // h
}

fn simulate(ground: &mut Vec<u8>, nrocks: usize, commands: &Vec<(fn(&u8) -> bool, fn(u8) -> u8)>) {
    let mut ip = 0;
    
    for irock in 0..nrocks {

        let mut block = Block::new(irock % NBLOCKS);
        let mut y = ground.len() + 3;

        if irock%NBLOCKS==0 {
            println!("ip:{ip:3}-{:3} irock:{irock}",commands.len());
        }

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
}

fn print_ground(ground: &Vec<u8>, nrows: Option<usize>) {
    let nrows = if let Some(n) = nrows {n} else {ground.len()};
    for (i, row) in ground.iter().enumerate().rev().take(nrows) {
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
    // assert_eq!(1514285714288, part2(include_str!("../assets/day17.test.txt")));
}
