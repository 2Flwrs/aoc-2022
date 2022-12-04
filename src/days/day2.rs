use crate::common::{input_filename, PuzzleStage};
use anyhow::Result;
use clap::Args;
use parse_display::{Display, FromStr};
use std::{cmp::Ordering, io::BufRead, str::FromStr};
use std::{fs::File, io::BufReader};

const DAY: usize = 2;

#[derive(Args, Debug)]
pub struct Day2Arg {
    stage: PuzzleStage,
    #[arg(short, long)]
    real: bool,
}

impl Day2Arg {
    pub(crate) fn run(self) -> Result<()> {
        println!("AoC Day {}", DAY);
        let path = input_filename(DAY, self.real);
        println!("using file {}", path.to_string_lossy());

        let r = File::open(path)?;
        let r = BufReader::new(r);

        let mut data = vec![];

        for line in r.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let row = Row::from_str(line.trim())?;
            data.push(row);
        }

        println!("Loaded {} sets of data", data.len());

        match self.stage {
            PuzzleStage::First => first_stage(&data),
            PuzzleStage::Second => second_stage(&data),
        }
    }
}

fn first_stage(data: &[Row]) -> Result<()> {
    let score = data.iter().map(Row::calc1).sum::<usize>();
    println!("Total score: {score}");
    Ok(())
}

fn second_stage(data: &[Row]) -> Result<(), anyhow::Error> {
    let score = data.iter().map(Row::calc2).sum::<usize>();
    println!("Total score: {score}");
    Ok(())
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{left} {right}")]
struct Row {
    left: Left,
    right: Right,
}

impl Row {
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
