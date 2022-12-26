use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, u64, u8},
    multi::separated_list1,
    IResult,
};

pub fn solve() -> String {
    let input = include_str!("../inputs/11.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input, false);
    (0..20).for_each(|_| round(&mut monkeys));
    monkeys.sort_by_key(|m| m.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product()
}

fn part2(input: &str) -> u64 {
    let mut monkeys = parse_monkeys(input, true);
    (0..10_000).for_each(|_| round(&mut monkeys));
    monkeys.sort_by_key(|m| m.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product()
}

fn parse_monkeys(input: &str, ridiculous: bool) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| {
            parse_monkey(s, ridiculous)
                .expect("failed to parse monkey")
                .1
        })
        .collect();
    if ridiculous {
        let modulo = monkeys.iter().map(|m| m.test).product();
        monkeys.iter_mut().for_each(|m| m.modulo = modulo);
    }
    monkeys
}

fn round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        monkeys[i].turn().iter().for_each(|(item, dest)| {
            monkeys[*dest].items.push(*item);
        });
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
    ridiculous: bool,
    modulo: u64,
}

impl Monkey {
    fn turn(&mut self) -> Vec<(u64, usize)> {
        self.inspections += self.items.len() as u64;
        let item_destinations: Vec<_> = self
            .items
            .iter()
            .map(|&item| {
                let mut item = self.increase_worry(item);
                item = self.decrease_worry(item);
                let dest = self.item_destination(item);
                (item, dest)
            })
            .collect();
        self.items.clear();
        item_destinations
    }

    fn increase_worry(&self, item: u64) -> u64 {
        match self.operation {
            Operation::Plus(OpAmount::Num(n)) => item + n,
            Operation::Plus(OpAmount::Old) => item + item,
            Operation::Times(OpAmount::Num(n)) => item * n,
            Operation::Times(OpAmount::Old) => item * item,
        }
    }

    fn decrease_worry(&self, item: u64) -> u64 {
        if self.ridiculous {
            item % self.modulo
        } else {
            // NB This rounds towards zero, as is required.
            item / 3
        }
    }

    fn item_destination(&self, item: u64) -> usize {
        if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Times(OpAmount),
    Plus(OpAmount),
}

#[derive(Copy, Clone, Debug)]
enum OpAmount {
    Old,
    Num(u64),
}

fn parse_monkey(i: &str, ridiculous: bool) -> IResult<&str, Monkey> {
    let (i, _) = tag("Monkey ")(i)?;
    let (i, _) = u8(i)?;
    let (i, _) = tag(":\n  Starting items: ")(i)?;
    let (i, items) = separated_list1(tag(", "), u64)(i)?;
    let (i, _) = tag("\n  Operation: new = old ")(i)?;
    let (i, op) = alt((tag("+ "), tag("* ")))(i)?;
    let (i, op_amount) = alt((digit1, tag("old")))(i)?;
    let (i, _) = tag("\n  Test: divisible by ")(i)?;
    let (i, test) = u64(i)?;
    let (i, _) = tag("\n    If true: throw to monkey ")(i)?;
    let (i, if_true) = u8(i)?;
    let (i, _) = tag("\n    If false: throw to monkey ")(i)?;
    let (i, if_false) = u8(i)?;
    let operation_amount = match op_amount {
        "old" => OpAmount::Old,
        n => OpAmount::Num(n.parse().unwrap()),
    };
    let operation = match op {
        "+ " => Operation::Plus(operation_amount),
        "* " => Operation::Times(operation_amount),
        _ => unreachable!(),
    };
    Ok((
        i,
        Monkey {
            items,
            operation,
            test,
            if_true: if_true as usize,
            if_false: if_false as usize,
            inspections: 0,
            ridiculous,
            modulo: 0,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 10605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 2713310158);
    }
}
