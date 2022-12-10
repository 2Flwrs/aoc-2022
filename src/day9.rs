use std::{collections::HashSet, ops::Not, str::FromStr};

use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug)]
struct Input {
    list: Vec<Move>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let list = s
            .lines()
            .filter_map(|x| {
                let x = x.trim();
                x.is_empty().not().then_some(x)
            })
            .map(|x| Ok(x.parse::<Move>()?))
            .collect::<Result<Vec<Move>>>()?;
        Ok(Self { list })
    }
}

#[derive(Display, FromStr, Clone, Copy, PartialEq, Eq)]
#[display("{direction} {steps}")]
struct Move {
    direction: Direction,
    steps: usize,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} {}>", self.direction, self.steps)
    }
}

#[derive(Display, FromStr, Clone, Copy, PartialEq, Eq)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

impl Direction {
    fn delta(&self) -> DeltaXY {
        match self {
            Direction::Up => DeltaXY::new(0, 1),
            Direction::Down => DeltaXY::new(0, -1),
            Direction::Left => DeltaXY::new(-1, 0),
            Direction::Right => DeltaXY::new(1, 0),
        }
    }
}

#[aoc_generator(day9)]
fn parse_data(input: &str) -> Result<Input> {
    Ok(input.parse()?)
}

#[aoc(day9, part1)]
fn part1(data: &Input) -> Result<usize> {
    let mut covered = HashSet::<PosXY>::new();

    let mut head = PosXY::default();
    let mut tail = PosXY::default();

    for m in data.list.iter().cloned() {
        let delta = m.direction.delta();
        for _ in 0..m.steps {
            head += delta;
            tail.move_towards(head);
            covered.insert(tail);
        }
    }

    Ok(covered.len())
}

const ROPE_LENGTH: usize = 10;

#[aoc(day9, part2)]
fn part2(data: &Input) -> Result<usize> {
    let mut covered = HashSet::<PosXY>::new();

    let mut rope = Vec::new();
    for _ in 0..ROPE_LENGTH {
        rope.push(PosXY::default());
    }

    for m in data.list.iter().cloned() {
        let delta = m.direction.delta();
        for _ in 0..m.steps {
            rope[0] += delta;
            for i in 1..ROPE_LENGTH {
                let target = rope[i - 1];
                rope[i].move_towards(target);
            }
            covered.insert(rope.last().unwrap().clone());
        }
        // println!("After move {m}");
        // plot(&rope, &covered);
        // println!();
    }

    Ok(covered.len())
}

#[allow(unused)]
fn plot(rope: &Vec<PosXY>, covered: &HashSet<PosXY>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for pos in rope.iter().chain(covered.iter()).cloned() {
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
    }

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let pos = PosXY::new(x, y);
            if let Some(i) = rope.iter().find_position(|p| pos == **p).map(|(i, _)| i) {
                match i {
                    0 => {
                        print!("H");
                    }
                    1..=9 => {
                        print!("{i}");
                    }
                    _ => panic!("unexpected"),
                }
            } else if covered.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day9.example.txt");
    const EXAMPLE2_INPUT: &str = include_str!("../input/2022/day9.example2.txt");

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 13);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 1);
        Ok(())
    }

    #[test]
    fn part2_example2() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE2_INPUT)?)?, 36);
        Ok(())
    }
}

use pos::*;
mod pos {
    use super::delta::*;
    use std::ops::*;

    #[derive(Hash, Clone, Copy, PartialEq, Eq, Default)]
    pub struct PosXY {
        pub x: isize,
        pub y: isize,
    }

    impl PosXY {
        pub fn new(x: isize, y: isize) -> Self {
            Self { x, y }
        }
        pub fn move_towards(&mut self, other: PosXY) {
            while (other - *self).max_norm() > 1 {
                *self += (other - *self).limit_to_1();
            }
        }
    }

    impl Add<DeltaXY> for PosXY {
        type Output = Self;
        fn add(self, rhs: DeltaXY) -> Self::Output {
            Self::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub<DeltaXY> for PosXY {
        type Output = Self;
        fn sub(self, rhs: DeltaXY) -> Self::Output {
            Self::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Sub<PosXY> for PosXY {
        type Output = DeltaXY;
        fn sub(self, rhs: PosXY) -> Self::Output {
            DeltaXY::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl AddAssign<DeltaXY> for PosXY {
        fn add_assign(&mut self, rhs: DeltaXY) {
            *self = *self + rhs;
        }
    }

    impl SubAssign<DeltaXY> for PosXY {
        fn sub_assign(&mut self, rhs: DeltaXY) {
            *self = *self - rhs;
        }
    }
}

use delta::*;
mod delta {
    use std::ops::*;

    #[derive(Hash, Clone, Copy, PartialEq, Eq, Default)]
    pub struct DeltaXY {
        pub x: isize,
        pub y: isize,
    }

    impl DeltaXY {
        pub fn new(x: isize, y: isize) -> Self {
            Self { x, y }
        }
        pub fn max_norm(&self) -> usize {
            let dx = self.x.abs() as usize;
            let dy = self.y.abs() as usize;
            dx.max(dy)
        }
        pub fn limit_to_1(&self) -> Self {
            Self::new(self.x.min(1).max(-1), self.y.min(1).max(-1))
        }
    }

    impl Add for DeltaXY {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for DeltaXY {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl AddAssign for DeltaXY {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl SubAssign for DeltaXY {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
}
