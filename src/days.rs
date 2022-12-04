mod day1;
mod day2;
mod day3;
mod day4;

use anyhow::{anyhow, Result};
use clap::Args;
use std::{fs::File, io::BufReader};

use crate::common::{input_filename, PuzzleStage};

#[derive(Args)]
pub struct PuzzleArgs {
    day: usize,
    stage: PuzzleStage,
    #[arg(short, long)]
    real: bool,
}

impl PuzzleArgs {
    pub(crate) fn run(self) -> Result<()> {
        println!("AoC Day {}", self.day);
        let path = input_filename(self.day, self.real);
        println!("using file {}", path.to_string_lossy());

        let r = File::open(path)?;
        let r = BufReader::new(r);

        match self.day {
            1 => day1::day1_run(r, self.stage),
            2 => day2::day2_run(r, self.stage),
            3 => day3::day3_run(r, self.stage), 
            4 => day4::day4_run(r, self.stage), 
            _ => Err(anyhow!("bad day: {}", self.day)),
        }?;

        Ok(())
    }
}
