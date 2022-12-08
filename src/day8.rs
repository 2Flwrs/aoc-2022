use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;
use itertools::process_results;
use std::str::FromStr;

struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Field {
    fn index(&self, row: usize, column: usize) -> usize {
        assert!(row < self.height);
        assert!(column < self.width);
        row * self.width + column
    }
}

fn char_to_height(c: char) -> Result<u8> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(anyhow!("bad char {c:#?}")),
    }
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = vec![];
        let mut width = None;
        let mut height = 0;

        for line in s.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let line = line.trim();
            let line_length = line.chars().count();
            if width.is_none() {
                width = Some(line_length);
            }
            if line_length != width.unwrap() {
                Err(anyhow!("differing widths"))?;
            }

            process_results(line.chars().map(char_to_height), |iter| data.extend(iter))?;

            height += 1;
        }

        let width = width.ok_or_else(|| anyhow!("no width"))?;
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

#[aoc_generator(day8)]
fn parse_data(input: &str) -> Result<Field> {
    Ok(input.parse()?)
}

#[aoc(day8, part1)]
fn part1(data: &Field) -> Result<usize> {
    let total_trees = data.width * data.height;
    let mut seen = bits![0].repeat(total_trees);

    for row in 0..data.height {
        let mut highest = data.data[data.index(row, 0)];
        seen.set(data.index(row, 0), true);

        for column in 1..data.width {
            let index = data.index(row, column);
            let current = data.data[index];
            if current > highest {
                seen.set(index, true);
                highest = current;
            }
        }
    }
    for row in 0..data.height {
        let mut highest = data.data[data.index(row, data.width - 1)];
        seen.set(data.index(row, data.width - 1), true);

        for column in (0..data.width - 1).rev() {
            let index = data.index(row, column);
            let current = data.data[index];
            if current > highest {
                seen.set(index, true);
                highest = current;
            }
        }
    }

    for column in 0..data.width {
        let mut highest = data.data[data.index(0, column)];
        seen.set(data.index(0, column), true);

        for row in 1..data.height {
            let index = data.index(row, column);
            let current = data.data[index];
            if current > highest {
                seen.set(index, true);
                highest = current;
            }
        }
    }
    for column in 0..data.width {
        let mut highest = data.data[data.index(data.height - 1, column)];
        seen.set(data.index(data.height - 1, column), true);

        for row in (0..data.height - 1).rev() {
            let index = data.index(row, column);
            let current = data.data[index];
            if current > highest {
                seen.set(index, true);
                highest = current;
            }
        }
    }

    Ok(seen.count_ones())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL_DIRECTIONS: [Direction; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

struct Position {
    index: usize,
    width: usize,
    height: usize,
}

impl Position {
    fn new(index: usize, width: usize, height: usize) -> Self {
        assert!(index < width * height);
        Self {
            index,
            width,
            height,
        }
    }

    fn from_pair(row: usize, column: usize, width: usize, height: usize) -> Self {
        assert!(row < height);
        assert!(column < width);
        let index = row * width + column;
        Self {
            index,
            width,
            height,
        }
    }

    fn with_new_position(&self, row: usize, column: usize) -> Self {
        Self::from_pair(row, column, self.width, self.height)
    }

    fn column(&self) -> usize {
        self.index % self.width
    }

    fn row(&self) -> usize {
        self.index / self.width
    }

    fn as_pair(&self) -> (usize, usize) {
        (self.row(), self.column())
    }

    fn step(&self, direction: Direction) -> Option<Position> {
        let (mut row, mut column) = self.as_pair();

        match direction {
            Direction::Up => {
                if row == 0 {
                    return None;
                }
                row -= 1;
            }
            Direction::Down => {
                if row == self.height - 1 {
                    return None;
                }
                row += 1;
            }
            Direction::Left => {
                if column == 0 {
                    return None;
                }
                column -= 1;
            }
            Direction::Right => {
                if column == self.width - 1 {
                    return None;
                }
                column += 1;
            }
        };

        Some(self.with_new_position(row, column))
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl Field {
    fn position_from_index(&self, index: usize) -> Position {
        Position::new(index, self.width, self.height)
    }

    fn part2_score(&self, index: usize) -> usize {
        Direction::ALL_DIRECTIONS
            .iter()
            .clone()
            .map(|&d| {
                let mut pos = self.position_from_index(index);
                let my_size = self.data[index];
                let mut count = 0;

                loop {
                    match pos.step(d) {
                        Some(p) => {
                            pos = p;
                            count += 1;
                            let other_size = self.data[pos.index()];
                            if other_size >= my_size {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                count
            })
            .product()
    }
}

#[aoc(day8, part2)]
fn part2(data: &Field) -> Result<usize> {
    (0..data.width * data.height)
        .map(|index| data.part2_score(index))
        .max()
        .ok_or_else(|| anyhow!("no max"))
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day8.example.txt");

    #[test]
    fn part1_initial_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 21);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 8);
        Ok(())
    }
}
