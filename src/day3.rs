use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day3)]
fn parse_data(input: &str) -> Result<Vec<Data>> {
    let mut data = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let entry = Data::from_line(&line)?;
        data.push(entry);
    }
    Ok(data)
}

#[aoc(day3, part1)]
fn part1(data: &[Data]) -> Result<usize> {
    let value = data
        .iter()
        .map(split_data)
        .map_ok(|(l, r)| matching(&l, &r) as usize)
        .sum::<Result<usize, _>>()?;

    Ok(value)
}

fn split_data(data: &Data) -> Result<(Vec<u8>, Vec<u8>)> {
    let data = &data.0;
    let len = data.len();
    if len % 2 != 0 {
        Err(anyhow!("not an even nnumber of chars"))?;
    }

    let left = data[0..(len / 2)].to_vec();
    let right = data[(len / 2)..len].to_vec();

    Ok((left, right))
}

fn matching(l: &[u8], r: &[u8]) -> u8 {
    let x = l
        .iter()
        .flat_map(|lch| if r.contains(lch) { Some(*lch) } else { None });

    let y = x.collect::<Vec<_>>();
    assert!(y.iter().all_equal());
    assert!(y.len() > 0);
    y[0]
}

#[aoc(day3, part2)]
fn part2(data: &[Data]) -> Result<usize> {
    let chunks = data.chunks_exact(3);
    if !chunks.remainder().is_empty() {
        return Err(anyhow!("not divided into even chunks"));
    }

    let total = chunks.map(calc_common).sum::<Result<usize, _>>()?;

    Ok(total)
}

fn calc_common(chunk: &[Data]) -> Result<usize> {
    if chunk.len() != 3 {
        return Err(anyhow!("bad chunk length"));
    }

    let overlapping = chunk
        .iter()
        .map(|Data(list)| list.iter().sorted().unique().cloned().collect::<Vec<_>>())
        .map(|x| x.into_iter().collect::<HashSet<_>>())
        .reduce(|l, r| l.intersection(&r).cloned().collect())
        .ok_or_else(|| anyhow!("no data?"))?;

    let answer = overlapping
        .into_iter()
        .exactly_one()
        .map_err(|_| anyhow!("more than one overlap"))?;

    Ok(answer as usize)
}

#[derive(Debug, Clone)]
struct Data(Vec<u8>);

impl Data {
    pub(crate) fn from_line(line: &str) -> Result<Self> {
        let all = line.chars().map(prio).collect::<Result<Vec<_>, _>>()?;
        Ok(Self(all))
    }
}

fn prio(c: char) -> Result<u8> {
    match c {
        'a'..='z' => Ok((c as u8 - 'a' as u8) + 1),
        'A'..='Z' => Ok((c as u8 - 'A' as u8) + 27),
        _ => Err(anyhow!("invalid character")),
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day3.example.txt");

    #[test]
    fn part1() {
        let data = super::parse_data(EXAMPLE_INPUT).unwrap();
        let answer = super::part1(&data).unwrap();
        assert_eq!(answer, 157);
    }

    #[test]
    fn part2() {
        let data = super::parse_data(EXAMPLE_INPUT).unwrap();
        let answer = super::part2(&data).unwrap();
        assert_eq!(answer, 70);
    }
}
