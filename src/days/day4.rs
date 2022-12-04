use crate::common::PuzzleStage;
use anyhow::Result;
use parse_display::{Display, FromStr};
use std::io::BufRead;

pub(crate) fn day4_run<R: BufRead>(r: R, stage: PuzzleStage) -> Result<()> {
    let answer = match stage {
        PuzzleStage::First => day4_stage1(r),
        PuzzleStage::Second => day4_stage2(r),
    }?;
    println!("{answer}");
    Ok(())
}

fn load_data<R: BufRead>(r: R) -> Result<Vec<Data>> {
    let mut data: Vec<Data> = vec![];
    for line in r.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        data.push(line.parse()?);
    }
    println!("Loaded {} sets of data", data.len());
    Ok(data)
}

fn day4_stage1<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;
    let count = data
        .iter()
        .filter_map(|Data(a, b)| (a.contains(b) || b.contains(a)).then_some(()))
        .count();

    Ok(format!("Complete overlaps: {count}"))
}

fn day4_stage2<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;
    let count = data
        .iter()
        .filter_map(|Data(a, b)| a.overlaps(b).then_some(()))
        .count();

    Ok(format!("Overlaps: {count}"))
}

#[derive(Display, FromStr, Clone, Copy)]
#[display("{0},{1}")]
struct Data(ElfRange, ElfRange);

impl Data {}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = format!("{}", self);
        write!(f, "Data({:?})", inner)
    }
}

#[derive(Debug, Display, FromStr, Clone, Copy)]
#[display("{low}-{high}")]
struct ElfRange {
    low: usize,
    high: usize,
}
impl ElfRange {
    fn contains(&self, other: &ElfRange) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn overlaps(&self, other: &ElfRange) -> bool {
        (self.low <= other.high && self.low >= other.low)
            || (self.high <= other.high && self.high >= other.low)
            || (self.low < other.low && self.high > other.high)
    }
}
