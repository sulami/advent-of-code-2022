use std::cmp::{Ordering, PartialOrd};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

pub fn solve() {
    let input = include_str!("../inputs/13.txt");
    println!("day 13-1: {}", part1(input));
    println!("day 13-2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let messages = parse_messages(input);
    (0..messages.len())
        .step_by(2)
        .enumerate()
        .map(|(n, i)| {
            if messages[i] < messages[i + 1] {
                n + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut messages = parse_messages(input);
    messages.push(parse_message("[[2]]").unwrap().1);
    messages.push(parse_message("[[6]]").unwrap().1);
    messages.sort();
    let a = messages
        .iter()
        .position(|m| *m == parse_message("[[2]]").unwrap().1)
        .expect("unable to find package");
    let b = messages
        .iter()
        .position(|m| *m == parse_message("[[6]]").unwrap().1)
        .expect("unable to find package");
    (a + 1) * (b + 1)
}

fn parse_messages(input: &str) -> Vec<Message> {
    input
        .split("\n\n")
        .flat_map(|pair| -> Vec<Message> {
            pair.lines()
                .map(|msg| parse_message(msg).expect("failed to parse message").1)
                .collect()
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Message {
    Atom(u8),
    List(Vec<Message>),
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Message::Atom(a), Message::Atom(b)) if a == b => None,
            (Message::Atom(a), Message::Atom(b)) => a.partial_cmp(b),
            (Message::List(_), Message::Atom(_)) => {
                self.partial_cmp(&Message::List(vec![other.clone()]))
            }
            (Message::Atom(_), Message::List(_)) => {
                Message::List(vec![self.clone()]).partial_cmp(other)
            }
            (Message::List(a), Message::List(b)) => {
                for i in 0..a.len().max(b.len()) {
                    if i >= a.len() && i < b.len() {
                        return Some(Ordering::Less);
                    }
                    if i >= b.len() && i < a.len() {
                        return Some(Ordering::Greater);
                    }
                    if let Some(result) = &a[i].partial_cmp(&b[i]) {
                        return Some(*result);
                    }
                }
                None
            }
        }
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn parse_message(s: &str) -> IResult<&str, Message> {
    alt((
        map(u8, Message::Atom),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_message), tag("]")),
            Message::List,
        ),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 140);
    }
}
