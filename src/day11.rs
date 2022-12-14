use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use evalexpr::{build_operator_tree, Node};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone, PartialEq, Eq)]
#[display(
    "{index}\n  {starting_items}\n  {op}\n  {test_div}\n    {target_true}\n    {target_false}"
)]
struct MonkeyInfo {
    #[display("Monkey {}:")]
    index: usize,
    #[display("Starting items: {}")]
    starting_items: ItemList,
    #[display("Operation: new = {}")]
    op: String,
    #[display("Test: divisible by {}")]
    test_div: i64,
    #[display("If true: throw to monkey {}")]
    target_true: usize,
    #[display("If false: throw to monkey {}")]
    target_false: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ItemList(Vec<i64>);

impl std::str::FromStr for ItemList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s
            .split(",")
            .map(|x| x.trim().parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;
        Ok(Self(list))
    }
}

impl std::fmt::Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().map(|x| format!("{x}")).join(", ").fmt(f)
    }
}

#[aoc_generator(day11)]
fn parse_data(input: &str) -> Result<Vec<MonkeyInfo>> {
    let mut monkeys = vec![];
    let mut lines = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            let monkey = lines.join("\n").trim().parse()?;
            monkeys.push(monkey);
            lines.clear();
        } else {
            lines.push(line);
        }
    }

    let last = lines.join("\n");
    if !last.trim().is_empty() {
        let monkey = lines.join("\n").trim().parse()?;
        monkeys.push(monkey);
    }

    Ok(monkeys)
}

// ============================================================================

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Node,
    test_div: i64,
    target_true: usize,
    target_false: usize,
    inspected_total: usize,
}

impl Monkey {
    fn from_info(info: &MonkeyInfo) -> Result<Self> {
        let items = info.starting_items.0.clone();
        let op = build_operator_tree(&info.op)?;
        let test_div = info.test_div;
        let target_true = info.target_true;
        let target_false = info.target_false;
        Ok(Self {
            items,
            op,
            test_div,
            target_true,
            target_false,
            inspected_total: 0,
        })
    }

    fn process_items(&mut self) -> Result<Vec<(usize, i64)>> {
        let mut thrown = vec![];

        for item in self.items.iter().cloned() {
            self.inspected_total += 1;

            let ctx = evalexpr::context_map! {
                "old" => item
            }?;

            let new_item = self.op.eval_int_with_context(&ctx)?;

            let new_item = new_item / 3;

            let target = if new_item % self.test_div == 0 {
                self.target_true
            } else {
                self.target_false
            };

            thrown.push((target, new_item));
        }
        self.items.clear();

        Ok(thrown)
    }

    fn fetch_item(&mut self, item: i64) {
        self.items.push(item);
    }

    fn inspected_total(&self) -> usize {
        self.inspected_total
    }
}

#[aoc(day11, part1)]
fn part1(info: &[MonkeyInfo]) -> Result<usize> {
    let mut monkeys = info
        .iter()
        .map(Monkey::from_info)
        .collect::<Result<Vec<_>, _>>()?;

    for _round in 0..20 {
        for monkey_num in 0..monkeys.len() {
            let thrown = monkeys[monkey_num].process_items()?;
            for (target, item) in thrown {
                monkeys
                    .get_mut(target)
                    .ok_or_else(|| anyhow!("bad index {target}"))?
                    .fetch_item(item);
            }
        }

        #[cfg(feature = "verbose")]
        {
            eprintln!("Round {}:", round + 1);
            for (num, m) in monkeys.iter().enumerate() {
                let list = m.items.iter().map(|i| format!("{i}")).join(", ");
                eprintln!("Monkey {num}: {list}");
            }
            eprintln!();
        }

        // dbg!(&monkeys);
    }

    Ok(monkeys
        .iter()
        .map(Monkey::inspected_total)
        .sorted()
        .rev()
        .take(2)
        .product())
}

// ============================================================================

#[derive(Debug)]
struct Item(HashMap<i64, i64>);

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self
            .0
            .iter()
            .sorted_by_key(|(k, _v)| **k)
            .map(|(div, val)| format!("{val}%{div}"))
            .join(", ");
        write!(f, "[{inner}]")
    }
}

impl Item {
    fn from_divs(initial: i64, divs: &[i64]) -> Self {
        Self(divs.iter().map(|&d| (d, initial.rem_euclid(d))).collect())
    }

    fn is_div_by(&self, div: i64) -> bool {
        self.0.get(&div).map(|x| *x % div == 0).unwrap()
    }

    fn operate(&mut self, op: &Node) -> Result<()> {
        // println!("## Operating on {self}");
        // println!(" # with {op}");
        for (&div, val) in &mut self.0 {
            let old = *val;
            let ctx = evalexpr::context_map! {
                "old" => old
            }?;
            let new = op.eval_int_with_context(&ctx)?;
            let rem = new.rem_euclid(div);
            *val = rem;

            // println!(" # old={old}  new={new}  == {rem} mod {div}");
        }
        // println!(" # Results in {self}");
        // println!();

        Ok(())
    }
}

#[derive(Debug)]
struct Monkey2 {
    items: Vec<Item>,
    op: Node,
    test_div: i64,
    target_true: usize,
    target_false: usize,
    inspected_total: usize,
}

impl Monkey2 {
    fn from_info(info: &MonkeyInfo, all_divs: &[i64]) -> Result<Self> {
        let items = info
            .starting_items
            .0
            .iter()
            .map(|&initial| Item::from_divs(initial, all_divs))
            .collect();
        let op = build_operator_tree(&info.op)?;
        let test_div = info.test_div;
        let target_true = info.target_true;
        let target_false = info.target_false;
        Ok(Self {
            items,
            op,
            test_div,
            target_true,
            target_false,
            inspected_total: 0,
        })
    }

    fn process_items(&mut self) -> Result<Vec<(usize, Item)>> {
        let mut thrown = vec![];

        for mut item in self.items.drain(..) {
            self.inspected_total += 1;

            item.operate(&self.op)?;

            let target = if item.is_div_by(self.test_div) {
                self.target_true
            } else {
                self.target_false
            };

            thrown.push((target, item));
        }
        self.items.clear();

        Ok(thrown)
    }

    fn catch_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn inspected_total(&self) -> usize {
        self.inspected_total
    }
}

#[aoc(day11, part2)]
fn part2(info: &[MonkeyInfo]) -> Result<usize> {
    let all_divs = info.iter().map(|m| m.test_div).collect::<Vec<_>>();

    let mut monkeys = info
        .iter()
        .map(|i| Monkey2::from_info(i, &all_divs))
        .collect::<Result<Vec<_>, _>>()?;

    // println!("== At start ==");
    // print_monkeys2(&monkeys);
    // println!();

    // let debug_list = [1, 20, 10000];
    for _round in 0..10000 {
        for monkey_num in 0..monkeys.len() {
            let thrown = monkeys[monkey_num].process_items()?;
            for (target, item) in thrown {
                monkeys
                    .get_mut(target)
                    .ok_or_else(|| anyhow!("bad index {target}"))?
                    .catch_item(item);
            }
        }

        #[cfg(feature = "verbose")]
        {
            eprintln!("Round {}:", round + 1);
            for (num, m) in monkeys.iter().enumerate() {
                let list = m.items.iter().map(|i| format!("{i}")).join(", ");
                eprintln!("Monkey {num}: {list}");
            }
            eprintln!();
        }

        // let round_number = _round + 1;
        // if debug_list.contains(&round_number) {
        //     println!("== After round {round_number} ==");
        //     print_monkeys2(&monkeys);
        //     println!();
        // }

        // dbg!(&monkeys);
    }

    Ok(monkeys
        .iter()
        .map(Monkey2::inspected_total)
        .sorted()
        .rev()
        .take(2)
        .product())
}

// fn print_monkeys2(monkeys: &[Monkey2]) {
//     for (i, m) in monkeys.iter().enumerate() {
//         println!("Monkey {i} inspected items {} times", m.inspected_total);
//         for item in &m.items {
//             println!("  item {item}",);
//         }
//     }
// }

// ============================================================================

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day11.example.txt");

    #[test]
    fn parse() -> Result<()> {
        let _info = parse_data(EXAMPLE_INPUT)?;
        Ok(())
    }

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data(EXAMPLE_INPUT)?)?, 10605);
        Ok(())
    }

    #[test]
    #[cfg_attr(
        not(feature = "long-running-tests"),
        ignore = "long running (use feature 'long-running-tests' to enable)"
    )]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data(EXAMPLE_INPUT)?)?, 2713310158);
        Ok(())
    }
}
