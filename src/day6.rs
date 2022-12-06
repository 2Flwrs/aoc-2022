use std::collections::VecDeque;

use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1, A_Tuples)]
fn part1_tuples(data: &str) -> Result<usize> {
    let index = data
        .chars()
        .tuple_windows()
        .enumerate()
        .find_map(|(ix, (a, b, c, d))| {
            let list = [a, b, c, d];
            list.into_iter().all_unique().then_some(ix + 4)
        })
        .ok_or_else(|| anyhow!("no start marker"))?;
    Ok(index)
}

#[aoc(day6, part1, B_Generic)]
fn part1_generic(data: &str) -> Result<usize> {
    find_marker_position(data, 4)
}

#[aoc(day6, part2)]
fn part2(data: &str) -> Result<usize> {
    find_marker_position(data, 14)
}

fn find_marker_position(data: &str, marker_length: usize) -> Result<usize> {
    let index = data
        .chars()
        .windows(marker_length)
        .enumerate()
        .find_map(|(ix, mut window)| window.all_unique().then_some(ix + marker_length))
        .ok_or_else(|| anyhow!("no start marker"))?;
    Ok(index)
}

// ---------------------------------------
// Implement cloned windows on an Iterator

trait IterWindowsExt<T: Clone, I: Iterator<Item = T>> {
    fn windows(self, size: usize) -> IterWindows<T, I>;
}

impl<T: Clone, I: Iterator<Item = T>> IterWindowsExt<T, I> for I {
    fn windows(self, size: usize) -> IterWindows<T, I> {
        IterWindows::new(self, size)
    }
}

#[derive(Debug)]
struct IterWindows<T: Clone, I: Iterator<Item = T>> {
    it: I,
    size: usize,
    buf: VecDeque<T>,
}

impl<T: Clone, I: Iterator<Item = T>> IterWindows<T, I> {
    fn new(it: I, size: usize) -> Self {
        let buf = VecDeque::with_capacity(size);
        Self { it, size, buf }
    }
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for IterWindows<T, I> {
    type Item = std::vec::IntoIter<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.pop_front(); // discard first element.
        while self.buf.len() < self.size {
            let Some(value) = self.it.next() else { return None;};
            self.buf.push_back(value)
        }
        Some(self.buf.iter().cloned().collect::<Vec<_>>().into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::{part1_tuples, part1_generic, part2};
    use anyhow::Result;

    #[test]
    fn part1_tuples_examples() -> Result<()> {
        assert_eq!(part1_tuples("abcdefghijklmnopqrt")?, 4);
        assert_eq!(part1_tuples("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 7);
        assert_eq!(part1_tuples("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
        assert_eq!(part1_tuples("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
        assert_eq!(part1_tuples("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
        assert_eq!(part1_tuples("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);
        Ok(())
    }

    #[test]
    fn part1_generic_examples() -> Result<()> {
        assert_eq!(part1_generic("abcdefghijklmnopqrt")?, 4);
        assert_eq!(part1_generic("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 7);
        assert_eq!(part1_generic("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
        assert_eq!(part1_generic("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
        assert_eq!(part1_generic("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
        assert_eq!(part1_generic("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);
        Ok(())
    }

    #[test]
    fn part2_examples() -> Result<()> {
        assert_eq!(part2("abcdefghijklmnopqrt")?, 14);
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg")?, 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);
        Ok(())
    }
}
