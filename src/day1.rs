use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

struct Data(Vec<usize>);

#[aoc_generator(day1)]
fn parse_data(input: &str) -> Result<Vec<Data>> {
    let mut data = vec![];
    let mut set = vec![];
    for line in input.lines() {
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

#[aoc(day1, part1)]
fn part1(data: &[Data]) -> Result<usize> {
    let max = data
        .iter()
        .map(|Data(set)| set.iter().sum::<usize>())
        .max()
        .ok_or_else(|| anyhow::anyhow!("no max?"))?;

    Ok(max)
}

#[aoc(day1, part2)]
fn part2(data: &[Data]) -> Result<usize> {
    let mut totals = data
        .iter()
        .map(|Data(set)| set.iter().sum::<usize>())
        .collect::<Vec<_>>();

    totals.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let sum3 = totals.iter().take(3).sum::<usize>();

    Ok(sum3)
}

#[cfg(test)]
mod test {
    const SHORT_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn part1() {
        let data = super::parse_data(SHORT_INPUT).unwrap();
        let answer = super::part1(&data).unwrap();
        assert_eq!(answer, 24000);
    }

    #[test]
    fn part2() {
        let data = super::parse_data(SHORT_INPUT).unwrap();
        let answer = super::part2(&data).unwrap();
        assert_eq!(answer, 45000);
    }
}
