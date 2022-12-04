use crate::common::PuzzleStage;
use anyhow::Result;
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
    let mut data = vec![];
    for line in r.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        data.push(Data());
        todo!()
    }
    println!("Loaded {} sets of data", data.len());
    Ok(data)
}

fn day4_stage1<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;
    dbg!(&data);
    todo!()
}

fn day4_stage2<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;
    dbg!(&data);
    todo!()
}

#[derive(Debug)]
struct Data();
