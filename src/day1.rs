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
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day1.example.txt");

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 24000);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 45000);
        Ok(())
    }
}
