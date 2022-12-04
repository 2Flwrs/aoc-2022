use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[aoc_generator(day4)]
fn parse_data(input: &str) -> Result<Vec<Data>> {
    let mut data: Vec<Data> = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        data.push(line.parse()?);
    }
    Ok(data)
}

#[aoc(day4, part1)]
fn part1(data: &[Data]) -> Result<usize> {
    let count = data
        .iter()
        .filter_map(|Data(a, b)| (a.contains(b) || b.contains(a)).then_some(()))
        .count();

    Ok(count)
}

#[aoc(day4, part2)]
fn part2(data: &[Data]) -> Result<usize> {
    let count = data
        .iter()
        .filter_map(|Data(a, b)| a.overlaps(b).then_some(()))
        .count();

    Ok(count)
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

#[cfg(test)]
mod test {
    const SHORT_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    #[test]
    fn part1() {
        let data = super::parse_data(SHORT_INPUT).unwrap();
        let answer = super::part1(&data).unwrap();
        assert_eq!(answer, 2);
    }

    #[test]
    fn part2() {
        let data = super::parse_data(SHORT_INPUT).unwrap();
        let answer = super::part2(&data).unwrap();
        assert_eq!(answer, 4);
    }
}
