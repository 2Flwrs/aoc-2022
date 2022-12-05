use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::{cmp::Ordering, str::FromStr};

#[aoc_generator(day2)]
fn parse_data(input: &str) -> Result<Vec<Data>> {
    let mut data = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let row = Data::from_str(line.trim())?;
        data.push(row);
    }
    Ok(data)
}

#[aoc(day2, part1)]
fn part1(data: &[Data]) -> Result<usize> {
    let score = data.iter().map(Data::calc1).sum::<usize>();
    Ok(score)
}

#[aoc(day2, part2)]
fn part2(data: &[Data]) -> Result<usize> {
    let score = data.iter().map(Data::calc2).sum::<usize>();
    Ok(score)
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{left} {right}")]
struct Data {
    left: Left,
    right: Right,
}

impl Data {
    fn calc1(&self) -> usize {
        let opponent = self.left.as_rps();
        let played = self.right.as_rps();

        let type_score = played.type_score();
        let result_score = result_score(played.partial_cmp(&opponent).unwrap());

        type_score + result_score
    }

    fn calc2(&self) -> usize {
        let opponent = self.left.as_rps();
        let ordering = self.right.as_ordering();

        let played = match ordering {
            Ordering::Less => opponent.loose_against(),
            Ordering::Equal => opponent,
            Ordering::Greater => opponent.win_against(),
        };

        let type_score = played.type_score();
        let result_score = result_score(ordering);

        type_score + result_score
    }
}

fn result_score(ordering: Ordering) -> usize {
    match ordering {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Left {
    A,
    B,
    C,
}

impl Left {
    fn as_rps(&self) -> Rps {
        match self {
            Left::A => Rps::R,
            Left::B => Rps::P,
            Left::C => Rps::S,
        }
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Right {
    X,
    Y,
    Z,
}

impl Right {
    fn as_rps(&self) -> Rps {
        match self {
            Right::X => Rps::R,
            Right::Y => Rps::P,
            Right::Z => Rps::S,
        }
    }

    fn as_ordering(&self) -> Ordering {
        match self {
            Right::X => Ordering::Less,
            Right::Y => Ordering::Equal,
            Right::Z => Ordering::Greater,
        }
    }
}

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
enum Rps {
    R,
    P,
    S,
}
impl Rps {
    fn loose_against(&self) -> Rps {
        match self {
            Rps::R => Rps::S,
            Rps::P => Rps::R,
            Rps::S => Rps::P,
        }
    }

    fn win_against(&self) -> Rps {
        match self {
            Rps::R => Rps::P,
            Rps::P => Rps::S,
            Rps::S => Rps::R,
        }
    }

    fn type_score(&self) -> usize {
        match self {
            Rps::R => 1,
            Rps::P => 2,
            Rps::S => 3,
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Rps::R => match other {
                Rps::R => Ordering::Equal,
                Rps::P => Ordering::Less,
                Rps::S => Ordering::Greater,
            },
            Rps::P => match other {
                Rps::R => Ordering::Greater,
                Rps::P => Ordering::Equal,
                Rps::S => Ordering::Less,
            },
            Rps::S => match other {
                Rps::R => Ordering::Less,
                Rps::P => Ordering::Greater,
                Rps::S => Ordering::Equal,
            },
        })
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day2.example.txt");

    #[test]
    fn part1() {
        let data = super::parse_data(EXAMPLE_INPUT).unwrap();
        let answer = super::part1(&data).unwrap();
        assert_eq!(answer, 15);
    }

    #[test]
    fn part2() {
        let data = super::parse_data(EXAMPLE_INPUT).unwrap();
        let answer = super::part2(&data).unwrap();
        assert_eq!(answer, 12);
    }
}
