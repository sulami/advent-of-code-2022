use fxhash::FxHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, one_of, u64},
    combinator::{all_consuming, map},
    sequence::tuple,
    IResult,
};

pub fn solve() -> String {
    let input = include_str!("../inputs/21.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> u64 {
    let monkeys: FxHashMap<String, Monkey> = input
        .lines()
        .map(|l| {
            all_consuming(parse_monkey)(l)
                .expect("failed to parse monkey")
                .1
        })
        .map(|m| (m.name.clone(), m))
        .collect();
    monkeys.get("root").unwrap().resolve(&monkeys)
}

fn part2(input: &str) -> u64 {
    let monkeys: FxHashMap<String, Monkey> = input
        .lines()
        .map(|l| {
            all_consuming(parse_monkey)(l)
                .expect("failed to parse monkey")
                .1
        })
        .map(|m| (m.name.clone(), m))
        .collect();
    monkeys.get("humn").unwrap().resolve_inverse(&monkeys)
}

#[derive(Clone, Debug)]
struct Monkey {
    name: String,
    number: Number,
}

impl Monkey {
    /// Returns the integer number of this monkey, resolving other
    /// monkeys if required.
    fn resolve(&self, monkeys: &FxHashMap<String, Self>) -> u64 {
        match &self.number {
            Number::Atom(n) => *n,
            Number::Add(a, b) => {
                let lhs = monkeys.get(a).unwrap().resolve(monkeys);
                let rhs = monkeys.get(b).unwrap().resolve(monkeys);
                lhs + rhs
            }
            Number::Subtract(a, b) => {
                let lhs = monkeys.get(a).unwrap().resolve(monkeys);
                let rhs = monkeys.get(b).unwrap().resolve(monkeys);
                lhs - rhs
            }
            Number::Multiply(a, b) => {
                let lhs = monkeys.get(a).unwrap().resolve(monkeys);
                let rhs = monkeys.get(b).unwrap().resolve(monkeys);
                lhs * rhs
            }
            Number::Divide(a, b) => {
                let lhs = monkeys.get(a).unwrap().resolve(monkeys);
                let rhs = monkeys.get(b).unwrap().resolve(monkeys);
                lhs / rhs
            }
        }
    }

    /// Returns the inverse solution of this monkey, resolving "up the
    /// tree."
    fn resolve_inverse(&self, monkeys: &FxHashMap<String, Self>) -> u64 {
        // Find the monkey above us that uses our result.
        let used_by = monkeys
            .values()
            .find(|m| match &m.number {
                Number::Add(a, b) => a == &self.name || b == &self.name,
                Number::Subtract(a, b) => a == &self.name || b == &self.name,
                Number::Multiply(a, b) => a == &self.name || b == &self.name,
                Number::Divide(a, b) => a == &self.name || b == &self.name,
                Number::Atom(_) => false,
            })
            .unwrap();

        // Find out on which side of the next monkey up we are.
        let (lhs, rhs) = match &used_by.number {
            Number::Add(a, b) => (a, b),
            Number::Subtract(a, b) => (a, b),
            Number::Multiply(a, b) => (a, b),
            Number::Divide(a, b) => (a, b),
            Number::Atom(_) => unreachable!(),
        };

        if used_by.name == "root" {
            // Switch to regular resolution down the other side once
            // we hit root.
            if lhs == &self.name {
                monkeys.get(rhs).unwrap().resolve(monkeys)
            } else {
                monkeys.get(lhs).unwrap().resolve(monkeys)
            }
        } else if lhs == &self.name {
            // Resolve downwards on the right hand side branch.
            let rhs = monkeys.get(rhs).unwrap().resolve(monkeys);
            match &used_by.number {
                Number::Add(_, _) => used_by.resolve_inverse(monkeys) - rhs,
                Number::Subtract(_, _) => used_by.resolve_inverse(monkeys) + rhs,
                Number::Multiply(_, _) => used_by.resolve_inverse(monkeys) / rhs,
                Number::Divide(_, _) => used_by.resolve_inverse(monkeys) * rhs,
                Number::Atom(_) => unreachable!(),
            }
        } else {
            // Resolve downwards the left hand side branch.
            let lhs = monkeys.get(lhs).unwrap().resolve(monkeys);
            match &used_by.number {
                Number::Add(_, _) => used_by.resolve_inverse(monkeys) - lhs,
                Number::Subtract(_, _) => lhs - used_by.resolve_inverse(monkeys),
                Number::Multiply(_, _) => used_by.resolve_inverse(monkeys) / lhs,
                Number::Divide(_, _) => lhs / used_by.resolve_inverse(monkeys),
                Number::Atom(_) => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Number {
    Atom(u64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let num = alt((
        map(u64, Number::Atom),
        map(
            tuple((
                alpha1::<&str, _>,
                tag(" "),
                one_of("+-*/"),
                tag(" "),
                alpha1,
            )),
            |(a, _, op, _, b)| -> Number {
                match op {
                    '+' => Number::Add(a.to_string(), b.to_string()),
                    '-' => Number::Subtract(a.to_string(), b.to_string()),
                    '*' => Number::Multiply(a.to_string(), b.to_string()),
                    '/' => Number::Divide(a.to_string(), b.to_string()),
                    _ => unreachable!(),
                }
            },
        ),
    ));
    map(tuple((alpha1, tag(": "), num)), |(name, _, num)| Monkey {
        name: name.to_string(),
        number: num,
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 152);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 301);
    }
}
