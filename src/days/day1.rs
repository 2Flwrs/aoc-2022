use crate::common::PuzzleStage;
use anyhow::Result;
use std::io::BufRead;

pub(crate) fn day1_run<R: BufRead>(r: R, stage: PuzzleStage) -> Result<()> {
    let answer = match stage {
        PuzzleStage::First => day1_stage1(r),
        PuzzleStage::Second => day1_stage2(r),
    }?;
    println!("{answer}");
    Ok(())
}

struct Data(Vec<usize>);

fn load_data<R: BufRead>(r: R) -> Result<Vec<Data>> {
    let mut data = vec![];
    let mut set = vec![];
    for line in r.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if !set.is_empty() {
                data.push(Data(set));
                set = vec![];
            }
        } else {
            let value: usize = line.trim().parse()?;
            set.push(value)
        }
    }
    if !set.is_empty() {
        data.push(Data(set));
    }
    Ok(data)
}

fn day1_stage1<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;

    let max = data
        .iter()
        .map(|Data(set)| set.iter().sum::<usize>())
        .max()
        .ok_or_else(|| anyhow::anyhow!("no max?"))?;

    Ok(format!("Max sum: {max}"))
}

fn day1_stage2<R: BufRead>(r: R) -> Result<String> {
    let data = load_data(r)?;

    let mut totals = data
        .iter()
        .map(|Data(set)| set.iter().sum::<usize>())
        .collect::<Vec<_>>();

    totals.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let sum3 = totals.iter().take(3).sum::<usize>();

    Ok(format!("Sum of top 3: {sum3}"))
}
