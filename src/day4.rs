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
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day4.example.txt");

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 2);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 4);
        Ok(())
    }
}
