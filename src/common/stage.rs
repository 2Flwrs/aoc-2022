use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PuzzleStage {
    First,
    Second,
}

impl FromStr for PuzzleStage {
    type Err = PuzzleStageFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "1" | "first" => Ok(Self::First),
            "2" | "second" => Ok(Self::Second),
            _ => Err(PuzzleStageFromStrErr::CantParse),
        }
    }
}

#[derive(Debug)]
pub(crate) enum PuzzleStageFromStrErr {
    CantParse,
}

impl Display for PuzzleStageFromStrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleStageFromStrErr::CantParse => "can't parse".fmt(f),
        }
    }
}

impl Error for PuzzleStageFromStrErr {}
