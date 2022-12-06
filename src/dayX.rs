use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

struct Data(Vec<usize>);

#[aoc_generator(dayX)]
fn parse_data(input: &str) -> Result<Vec<Data>> {
    let mut data = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        todo!();
    }
    Ok(data)
}

#[aoc(dayX, part1)]
fn part1(data: &[Data]) -> Result<usize> {
    todo!()
}

#[aoc(dayX, part2)]
fn part2(data: &[Data]) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = "example"; // --or--  include_str!("../input/2022/dayX.example.txt");

    #[test]
    #[ignore = "not implemented yet"]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 0);
        Ok(())
    }

    #[test]
    #[ignore = "not implemented yet"]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 0);
        Ok(())
    }
}
