use anyhow::Result;
#[allow(unused)]
use aoc_runner_derive::{aoc, aoc_generator};

struct Data(Vec<usize>);

// #[aoc_generator(dayX)]
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

// #[aoc(dayX, part1)]
fn part1(data: &[Data]) -> Result<usize> {
    todo!()
}

// #[aoc(dayX, part2)]
fn part2(data: &[Data]) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod test {
    const SHORT_INPUT: &str = "test\ntest\n";

    #[test]
    fn part1() {
        let data = super::parse_data(SHORT_INPUT).unwrap();
        let answer = super::part1(&data).unwrap();
        assert_eq!(answer, 1234);
    }

    // #[test]
    // fn part2() {
    //     let data = super::parse_data(SHORT_INPUT).unwrap();
    //     let answer = super::part2(&data).unwrap();
    //     assert_eq!(answer, 1234);
    // }
}
