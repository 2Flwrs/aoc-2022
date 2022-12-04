use crate::common::{input_filename, PuzzleStage};
use anyhow::Result;
use clap::Args;
use std::io::BufRead;
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

            data.push(line);
        }

        println!("Loaded {} sets of data", data.len());
        dbg!(&data);

        match self.stage {
            PuzzleStage::First => first_stage(&data),
            PuzzleStage::Second => second_stage(&data),
        }
    }
}

fn first_stage(_data: &[String]) -> Result<()> {
    todo!()
}

fn second_stage(_data: &[String]) -> Result<(), anyhow::Error> {
    todo!()
}
