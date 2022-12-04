use crate::common::{input_filename, PuzzleStage};
use anyhow::Result;
use clap::Args;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

const DAY: usize = 1;

#[derive(Args, Debug)]
pub struct Day1Arg {
    stage: PuzzleStage,
    #[arg(short, long)]
    real: bool,
}

impl Day1Arg {
    pub(crate) fn run(self) -> Result<()> {
        println!("AoC Day {}", DAY);
        let path = input_filename(DAY, self.real);
        println!("using file {}", path.to_string_lossy());

        let r = File::open(path)?;
        let r = BufReader::new(r);

        let mut data = vec![];
        let mut set = vec![];

        for line in r.lines() {
            let line = line?;
            if line.trim().is_empty() {
                if !set.is_empty() {
                    data.push(set);
                    set = vec![];
                }
            } else {
                let value: usize = line.trim().parse()?;
                set.push(value)
            }
        }
        if !set.is_empty() {
            data.push(set);
        }

        println!("Loaded {} sets of data", data.len());

        match self.stage {
            PuzzleStage::First => first_stage(&data),
            PuzzleStage::Second => second_stage(&data),
        }
    }
}

fn first_stage(data: &[Vec<usize>]) -> Result<()> {
    let max = data
        .iter()
        .map(|set| set.iter().sum::<usize>())
        .max()
        .ok_or_else(|| anyhow::anyhow!("no max?"))?;

    println!("Max sum: {max}");
    Ok(())
}

fn second_stage(data: &[Vec<usize>]) -> Result<(), anyhow::Error> {
    let mut totals = data
        .iter()
        .map(|set| set.iter().sum::<usize>())
        .collect::<Vec<_>>();

    totals.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let sum3 = totals.iter().take(3).sum::<usize>();

    println!("Sum of top 3: {sum3}");
    Ok(())
}
