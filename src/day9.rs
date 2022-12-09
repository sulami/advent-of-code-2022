use std::collections::HashSet;
use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../inputs/09.txt");
    println!("day 9-1: {}", part1(input));
    println!("day 9-2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut rope = Rope::new();
    input
        .lines()
        .map(|l| l.parse().expect("failed to parse move"))
        .for_each(|m| rope.move_head(&m));
    rope.tail_history.len()
}

fn part2(_input: &str) -> u32 {
    0
}

enum Move {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("U", n)) => Ok(Self::Up(n.parse().map_err(|_| "invalid move distance")?)),
            Some(("D", n)) => Ok(Self::Down(n.parse().map_err(|_| "invalid move distance")?)),
            Some(("L", n)) => Ok(Self::Left(n.parse().map_err(|_| "invalid move distance")?)),
            Some(("R", n)) => Ok(Self::Right(n.parse().map_err(|_| "invalid move distance")?)),
            _ => Err("invalid move"),
        }
    }
}

struct Rope {
    head: (i16, i16),
    tail: (i16, i16),
    tail_history: HashSet<(i16, i16)>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
            tail_history: HashSet::from([(0, 0)]),
        }
    }

    fn move_tail(&mut self) {
        let (x, y) = distance(self.head, self.tail);
        if x.abs() > 1 || y.abs() > 1 {
            if x == 0 {
                // Direct vertical move.
                if y > 0 {
                    self.tail.1 += 1;
                } else {
                    self.tail.1 -= 1;
                }
            } else if y == 0 {
                // Direct horizontal move.
                if x > 0 {
                    self.tail.0 += 1;
                } else {
                    self.tail.0 -= 1;
                }
            } else if x > 1 {
                // Diagonally to the right.
                self.tail.0 += 1;
                if y > 0 {
                    self.tail.1 += 1;
                } else {
                    self.tail.1 -= 1;
                }
            } else if x < 1 {
                // Diagonally to the left.
                self.tail.0 -= 1;
                if y > 0 {
                    self.tail.1 += 1;
                } else {
                    self.tail.1 -= 1;
                }
            } else if y > 1 {
                // Diagonally up.
                self.tail.1 += 1;
                if x > 0 {
                    self.tail.0 += 1;
                } else {
                    self.tail.0 -= 1;
                }
            } else if y < 1 {
                // Diagonally down.
                self.tail.1 -= 1;
                if x > 0 {
                    self.tail.0 += 1;
                } else {
                    self.tail.0 -= 1;
                }
            }
            self.tail_history.insert(self.tail);
        }
    }

    fn move_head(&mut self, m: &Move) {
        match m {
            Move::Up(n) => (0..*n).for_each(|_| {
                self.head.1 += 1;
                self.move_tail();
            }),
            Move::Down(n) => (0..*n).for_each(|_| {
                self.head.1 -= 1;
                self.move_tail();
            }),
            Move::Left(n) => (0..*n).for_each(|_| {
                self.head.0 -= 1;
                self.move_tail();
            }),
            Move::Right(n) => (0..*n).for_each(|_| {
                self.head.0 += 1;
                self.move_tail();
            }),
        }
    }
}

/// Returns the maximum 4-directional distance. Means overlapping is
/// 0, off by one is 1, off diagnoally is still 1. Maps directly to
/// whether the tail needs to move.
fn distance(head: (i16, i16), tail: (i16, i16)) -> (i16, i16) {
    (head.0 - tail.0, head.1 - tail.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13)
    }
}
