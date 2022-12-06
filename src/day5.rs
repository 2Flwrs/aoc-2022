use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

struct Data {
    stacks: CrateStack,
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
struct CrateStack(Vec<Vec<char>>);

impl CrateStack {
    fn apply_instruction_part1(&mut self, instruction: Instruction) -> Result<()> {
        for _ in 0..instruction.count {
            let from = self
                .0
                .get_mut(instruction.from)
                .ok_or_else(|| anyhow!("from index too large: {}", instruction.from))?;

            let Some(e) = from.pop() else {Err(anyhow!("selected stack is empty"))?};

            let to = self
                .0
                .get_mut(instruction.to)
                .ok_or_else(|| anyhow!("to index too large: {}", instruction.to))?;

            to.push(e);
        }

        Ok(())
    }

    fn apply_instruction_part2(&mut self, instruction: Instruction) -> Result<()> {
        let from = self
            .0
            .get_mut(instruction.from)
            .ok_or_else(|| anyhow!("from index too large: {}", instruction.from))?;

        let mut tmp = vec![];
        for _ in 0..instruction.count {
            let Some(e) = from.pop() else {Err(anyhow!("selected stack is empty"))?};
            tmp.push(e);
        }

        let to = self
            .0
            .get_mut(instruction.to)
            .ok_or_else(|| anyhow!("to index too large: {}", instruction.to))?;

        to.extend(tmp.into_iter().rev());

        Ok(())
    }

    fn get_tops_as_string(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.last().cloned().unwrap_or(' '))
            .collect()
    }
}

#[derive(Display, FromStr, Clone, Copy, PartialEq, Eq)]
#[display("move {count} from {from} to {to}")]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{self}").fmt(f)
    }
}

#[derive(Display, FromStr)]
#[display("[{0}]")]
struct Crate(char);

struct StackSlice(Vec<Option<char>>);

impl FromStr for StackSlice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s
            .chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| {
                let v = chunk.collect::<String>();
                if let Ok(c) = v.trim().parse::<Crate>() {
                    Ok(Some(c.0))
                } else if v.trim().is_empty() {
                    Ok(None)
                } else {
                    Err(())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(StackSlice(result))
    }
}

#[aoc_generator(day5)]
fn parse_data(input: &str) -> Result<Data> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut instructions = vec![];

    enum Mode {
        Crates,
        Empty,
        Instructions,
    }

    let mut mode = Mode::Crates;

    for line in input.lines() {
        match mode {
            Mode::Crates => {
                if let Ok(slice) = line.parse::<StackSlice>() {
                    // Extend stacks list, if needed
                    while stacks.len() < slice.0.len() {
                        stacks.push(vec![]);
                    }

                    for (index, c) in slice.0.into_iter().enumerate() {
                        if let Some(c) = c {
                            stacks[index].push(c);
                        }
                    }
                } else if is_label_list(line, stacks.len()) {
                    mode = Mode::Empty;
                } else {
                    Err(anyhow!("unexpected line: {line:?}"))?
                }
            }
            Mode::Empty => {
                if line.is_empty() {
                    mode = Mode::Instructions;
                } else {
                    Err(anyhow!("expected empty line"))?
                }
            }
            Mode::Instructions => {
                let mut instr: Instruction = line.parse()?;
                if instr.from == 0 || instr.to == 0 {
                    Err(anyhow!("index not 1-based"))?
                };
                instr.from -= 1;
                instr.to -= 1;
                instructions.push(instr);
            }
        }
    }

    for v in stacks.iter_mut() {
        v.reverse();
    }

    let stacks = CrateStack(stacks);
    Ok(Data {
        stacks,
        instructions,
    })
}

fn is_label_list(_line: &str, _len: usize) -> bool {
    //todo!()
    true
}
#[aoc(day5, part1)]
fn part1(data: &Data) -> Result<String> {
    let mut stacks = data.stacks.clone();
    for i in data.instructions.iter().cloned() {
        stacks.apply_instruction_part1(i)?;
    }

    Ok(stacks.get_tops_as_string())
}

#[aoc(day5, part2)]
fn part2(data: &Data) -> Result<String> {
    let mut stacks = data.stacks.clone();
    for i in data.instructions.iter().cloned() {
        stacks.apply_instruction_part2(i)?;
    }

    Ok(stacks.get_tops_as_string())
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day5.example.txt");

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, "CMZ".to_string());
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, "MCD".to_string());
        Ok(())
    }
}
