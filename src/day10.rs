use std::str::FromStr;

use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::vec::BitVec;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Program(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<Instruction>, _>>()?,
        ))
    }
}

#[derive(Display, FromStr, Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    #[display("noop")]
    NoOp,
    #[display("addx {0}")]
    AddX(i32),
}

impl Instruction {
    fn dt_dx(&self) -> (i32, i32) {
        match *self {
            Instruction::NoOp => (1, 0),
            Instruction::AddX(dx) => (2, dx),
        }
    }
}

#[aoc_generator(day10)]
fn parse_data(input: &str) -> Result<Program> {
    Ok(input.parse()?)
}

const TARGET_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

#[aoc(day10, part1)]
fn part1(pgm: &Program) -> Result<i32> {
    let mut x = 1;
    let mut cycle = 1;
    let mut targets = TARGET_CYCLES.iter().cloned();

    let mut value = 0;

    let mut current_target = targets.next();
    for ins in pgm.0.iter().cloned() {
        let (dt, dx) = ins.dt_dx();
        let new_cycle = cycle + dt;

        if current_target.map_or(false, |targ| new_cycle > targ) {
            let targ = current_target.unwrap();
            let strength = x * targ;
            // println!(
            //     "Before applying {ins}, advancing to {new_cycle}, add {x} * {targ} = {strength}"
            // );
            value += strength;
            current_target = targets.next();
        }

        cycle = new_cycle;
        x += dx;
    }

    Ok(value)
}

#[derive(PartialEq, Eq, Clone)]
struct Screen {
    data: BitVec,
    width: usize,
}

impl std::fmt::Debug for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width;
        let numbits = self.data.len();
        write!(f, "Screen {{ width: {width}, data: <{numbits} bits>}}")
    }
}

impl Screen {
    #[cfg(test)]
    pub(crate) fn from_lines(vec: &[String]) -> Result<Screen> {
        use anyhow::anyhow;

        let width = vec.first().ok_or_else(|| anyhow!("no data"))?.len();
        if !vec.iter().map(|s| s.len()).all_equal() {
            Err(anyhow!("not all equal length"))?;
        }

        let data = itertools::process_results(
            vec.iter().flat_map(|s| {
                s.chars().map(|c| match c {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    _ => Err(anyhow!("bad char {c:?}")),
                })
            }),
            |iter| BitVec::from_iter(iter),
        )?;

        Ok(Self { data, width })
    }

    fn new(width: usize) -> Screen {
        let data = BitVec::new();
        Self { data, width }
    }

    fn push(&mut self, value: bool) {
        self.data.push(value)
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Screen:")?;
        let iter = self.data.iter().chunks(self.width);
        for line in &iter {
            let s = line
                .map(|x| if *x { '\u{2588}' } else { ' ' })
                .collect::<String>();
            writeln!(f, "  |{s}|")?;
        }
        writeln!(f, "End.")
    }
}

struct Cpu {
    screen: Screen,
    screen_pos: i32,
    x: i32,
}
impl Cpu {
    fn new(screen_width: usize) -> Self {
        let screen = Screen::new(screen_width);
        let screen_pos = 0;
        let x = 1;
        Self {
            screen,
            screen_pos,
            x,
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NoOp => self.do_cycle(),
            Instruction::AddX(dx) => {
                self.do_cycle();
                self.do_cycle();
                self.x += dx;
            }
        }
    }

    fn get_screen(&self) -> Screen {
        self.screen.clone()
    }

    fn do_cycle(&mut self) {
        let bit = (self.x - self.screen_pos).abs() <= 1;
        self.screen.push(bit);

        self.screen_pos += 1;
        self.screen_pos = self.screen_pos % (self.screen.width as i32);
    }
}

#[aoc(day10, part2)]
fn part2(data: &Program) -> Result<Screen> {
    let mut cpu = Cpu::new(40);
    for ins in data.0.iter().cloned() {
        cpu.run_instruction(ins);
    }
    Ok(cpu.get_screen())
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day10.example.txt");

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 13140);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        let result = part2(&parse_data(EXAMPLE_INPUT)?)?;

        let expected = super::Screen::from_lines(&[
            String::from("##..##..##..##..##..##..##..##..##..##.."),
            String::from("###...###...###...###...###...###...###."),
            String::from("####....####....####....####....####...."),
            String::from("#####.....#####.....#####.....#####....."),
            String::from("######......######......######......####"),
            String::from("#######.......#######.......#######....."),
        ])?;

        println!("Result: {result}");
        println!("Expected: {expected}");
        assert_eq!(result, expected);
        Ok(())
    }
}
