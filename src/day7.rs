use std::collections::HashMap;

use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Debug, Default, Clone)]
struct InputTreeLevel(HashMap<String, TreeNode>);

impl InputTreeLevel {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Debug)]
struct InputTree(InputTreeLevel);

#[derive(Debug, Clone)]
enum TreeNode {
    Dir(InputTreeLevel),
    File(usize),
}

#[derive(Debug, Display, FromStr)]
enum InputLine {
    #[display("$ cd /")]
    CdRoot(),
    #[display("$ cd ..")]
    CdUp(),
    #[display("$ cd {0}")]
    Cd(String),
    #[display("$ ls")]
    Ls(),
    #[display("{0} {1}")]
    File(usize, String),
    #[display("dir {0}")]
    Dir(String),
}

#[aoc_generator(day7)]
fn parse_data(input: &str) -> Result<InputTree> {
    let input_lines = input.lines().map(|l| l.trim().parse::<InputLine>());

    let mut tree = InputTreeLevel::new();
    let mut path = vec![];

    for line in input_lines {
        let line = line?;

        match line {
            InputLine::CdRoot() => {
                path.clear();
            }
            InputLine::CdUp() => {
                let element = path.pop();
                assert!(element.is_some());
            }
            InputLine::Cd(dir) => {
                path.push(dir);
            }
            InputLine::Ls() => {
                // Ignore
            }
            InputLine::File(size, file_name) => {
                let here = find_dir(&path, &mut tree)?;
                here.0.insert(file_name, TreeNode::File(size));
            }
            InputLine::Dir(dir_name) => {
                let here = find_dir(&path, &mut tree)?;
                here.0
                    .insert(dir_name, TreeNode::Dir(InputTreeLevel::new()));
            }
        };
    }

    Ok(InputTree(tree))
}

fn find_dir<'a>(
    path: &'a [String],
    tree: &'a mut InputTreeLevel,
) -> Result<&'a mut InputTreeLevel> {
    let mut here = tree;
    for dir in path.iter().cloned() {
        let TreeNode::Dir(next) = here.0.get_mut(&dir).ok_or_else(||anyhow!("could not navigate to path {path:?}"))? else {Err(anyhow!("wrong type of node"))?};

        here = next;
    }

    Ok(here)
}

#[derive(Debug)]
struct SizeCalcLevel {
    size: usize,
    inner: HashMap<String, SizeCalcLevel>,
}

impl From<InputTreeLevel> for SizeCalcLevel {
    fn from(i: InputTreeLevel) -> Self {
        let mut total = 0;
        let inner =
            i.0.into_iter()
                .filter_map(|(name, node)| match node {
                    TreeNode::Dir(inner) => {
                        let inner = Self::from(inner);
                        total += inner.size;
                        Some((name, inner))
                    }
                    TreeNode::File(size) => {
                        total += size;
                        None
                    }
                })
                .collect();
        Self { size: total, inner }
    }
}

impl SizeCalcLevel {
    fn get_sum_le(&self, limit: usize) -> usize {
        let mut total = if self.size <= limit { self.size } else { 0 };

        for (_, inner) in self.inner.iter() {
            total += inner.get_sum_le(limit);
        }

        total
    }
}

#[aoc(day7, part1)]
fn part1(data: &InputTree) -> Result<usize> {
    let input_data = data.0.clone();
    let tree: SizeCalcLevel = input_data.into();
    Ok(tree.get_sum_le(100000))
}

impl SizeCalcLevel {
    fn find_smallest_ge(&self, needed: usize) -> Option<usize> {
        if self.size < needed {
            return None;
        }

        let mut best = self.size;

        for (_, inner) in self.inner.iter() {
            if let Some(other) = inner.find_smallest_ge(needed) {
                if other < best {
                    best = other;
                }
            }
        }

        Some(best)
    }
}

#[aoc(day7, part2)]
fn part2(data: &InputTree) -> Result<usize> {
    let tree: SizeCalcLevel = data.0.clone().into();

    let total_disk = 70000000;
    let needed_disk = 30000000;
    let root_size = tree.size;
    let available_disk = total_disk - root_size;
    let needed_cleanup = needed_disk - available_disk;

    tree.find_smallest_ge(needed_cleanup)
        .ok_or_else(|| anyhow!("no candidate found"))
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day7.example.txt");

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 95437);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 24933642);
        Ok(())
    }
}
