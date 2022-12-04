use crate::common::{input_filename, PuzzleStage};
use anyhow::{anyhow, Result};
use clap::Args;
use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};
use std::{fs::File, io::BufReader};

const DAY: usize = 3;

#[derive(Args, Debug)]
pub struct Day3Arg {
    stage: PuzzleStage,
    #[arg(short, long)]
    real: bool,
}

impl Day3Arg {
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

            let entry = Data::from_line(&line)?;
            data.push(entry);
        }

        println!("Loaded {} sets of data", data.len());

        match self.stage {
            PuzzleStage::First => first_stage(&data),
            PuzzleStage::Second => second_stage(&data),
        }
    }
}

fn first_stage(_data: &[Data]) -> Result<()> {
    let value = _data
        .iter()
        .map(split_data)
        .map_ok(|(l, r)| matching(&l, &r) as usize)
        .sum::<Result<usize, _>>()?;
    println!("Sum of prio: {value}");
    Ok(())
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

fn second_stage(data: &[Data]) -> Result<(), anyhow::Error> {
    let chunks = data.chunks_exact(3);
    if !chunks.remainder().is_empty() {
        return Err(anyhow!("not divided into even chunks"));
    }

    let total = chunks.map(calc_common).sum::<Result<usize, _>>()?;

    println!("Total badge prio: {total}");
    Ok(())
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
