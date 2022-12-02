use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt, value},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw
}

#[derive(Debug, Copy, Clone)]
struct Game(Move, Move);

impl Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Game {
    fn score(&self) -> u32 {
        self.outcome() + self.1.score()
    }

    fn outcome(&self) -> u32 {
        //    R P S
        //  P 0 3 6   0 1 2
        //  R 3 6 0   1 2 0
        //  S 6 0 3   2 0 1
        let p2 = match self.1 {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissors => 2,
        };
        let p1 = match self.0 {
            Move::Paper => 0,
            Move::Rock => 1,
            Move::Scissors => 2,
        };
        3 * ((p1 + p2) % 3)
    }
}

pub fn part1(input: &str) -> u32 {
    fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        fn play(input: &str) -> IResult<&str, Move> {
            alt((
                value(Move::Rock, alt((char('A'), char('X')))),
                value(Move::Paper, alt((char('B'), char('Y')))),
                value(Move::Scissors, alt((char('C'), char('Z')))),
            ))(input)
        }
        let game = map(
            terminated(separated_pair(play, char(' '), play), opt(char('\n'))),
            |(m0, m1)| Game(m0, m1),
        );
        many1(game)(input)
    }

    parse(input).unwrap().1.iter().map(|g| g.score()).sum()
}

pub fn part2(input: &str) -> u32 {
    fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        fn play(input: &str) -> IResult<&str, Move> {
            alt((
                value(Move::Rock, char('A')),
                value(Move::Paper, char('B')),
                value(Move::Scissors, char('C')),
            ))(input)
        }
        fn outcome(input: &str) -> IResult<&str, Outcome> {
            alt((
                value(Outcome::Lose, char('X')),
                value(Outcome::Draw, char('Y')),
                value(Outcome::Win, char('Z')),
            ))(input)
        }
        let game = map(
            terminated(separated_pair(play, char(' '), outcome), opt(char('\n'))),
            |(m0, outcome)| {
                //    W D L
                //    0 1 2
                // S1 0 1 2
                // P2 1 2 0
                // R0 2 0 1 
                let a:u8 = match outcome {
                    Outcome::Win => 0,
                    Outcome::Lose => 2,
                    Outcome::Draw => 1,
                };
                let b:u8= match m0 {
                    Move::Rock => 2,
                    Move::Paper => 1,
                    Move::Scissors => 0,
                };
                let m1 = match (a+b)%3 {
                    0 => Move::Rock,
                    1 => Move::Scissors,
                    2 => Move::Paper,
                    _ => unreachable!()
                };

                Game(m0, m1)
            },
        );
        many1(game)(input)
    }

    parse(input).unwrap().1.iter().map(|g| g.score()).sum()
}

#[test]
fn day2() {
    assert_eq!(15, part1(include_str!("../assets/day2.test.txt")));
    assert_eq!(12, part2(include_str!("../assets/day2.test.txt")));
}
