use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};

use crate::common::PuzzleStage;

pub(crate) fn day3_run<R: BufRead>(r: R, stage: PuzzleStage) -> Result<()> {
    let answer = match stage {
        PuzzleStage::First => day3_stage1(r),
        PuzzleStage::Second => day3_stage2(r),
    }?;
    println!("{answer}");
    Ok(())
}

fn load_data<R: BufRead>(r: R) -> Result<Vec<Data>> {
    let mut data = vec![];
    for line in r.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let entry = Data::from_line(&line)?;
        data.push(entry);
    }
    println!("Loaded {} sets of data", data.len());
    Ok(data)
}

fn day3_stage1<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;

    let value = data
        .iter()
        .map(split_data)
        .map_ok(|(l, r)| matching(&l, &r) as usize)
        .sum::<Result<usize, _>>()?;

    Ok(format!("Sum of prio: {value}"))
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

fn day3_stage2<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;
    let chunks = data.chunks_exact(3);
    if !chunks.remainder().is_empty() {
        return Err(anyhow!("not divided into even chunks"));
    }

    let total = chunks.map(calc_common).sum::<Result<usize, _>>()?;

    Ok(format!("Total badge prio: {total}"))
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
