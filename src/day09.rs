use fxhash::FxHashSet;
use std::str::FromStr;

pub fn solve() -> String {
    let input = include_str!("../inputs/09.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> usize {
    let mut rope = Rope::new(2);
    input
        .lines()
        .map(|l| l.parse().expect("failed to parse move"))
        .for_each(|m| rope.move_head(&m));
    rope.tail_history.len()
}

fn part2(input: &str) -> usize {
    let mut rope = Rope::new(10);
    input
        .lines()
        .map(|l| l.parse().expect("failed to parse move"))
        .for_each(|m| rope.move_head(&m));
    rope.tail_history.len()
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

type Knot = (i16, i16);

struct Rope {
    knots: Vec<Knot>,
    tail_history: FxHashSet<Knot>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        assert!(num_knots > 0);
        let mut tail_history = FxHashSet::default();
        tail_history.insert((0, 0));
        Self {
            knots: vec![(0, 0); num_knots],
            tail_history,
        }
    }

    fn move_head(&mut self, m: &Move) {
        match m {
            Move::Up(n) => {
                for _ in 0..*n {
                    let mut head = self.knots.get_mut(0).unwrap();
                    head.1 += 1;
                    for idx in 0..self.knots.len() - 1 {
                        let head = *self.knots.get(idx).unwrap();
                        let tail = self.knots.get_mut(idx + 1).unwrap();
                        move_tail(&head, tail);
                    }
                    self.tail_history.insert(*self.knots.last().unwrap());
                }
            }
            Move::Down(n) => {
                for _ in 0..*n {
                    let mut head = self.knots.get_mut(0).unwrap();
                    head.1 -= 1;
                    for idx in 0..self.knots.len() - 1 {
                        let head = *self.knots.get(idx).unwrap();
                        let tail = self.knots.get_mut(idx + 1).unwrap();
                        move_tail(&head, tail);
                    }
                    self.tail_history.insert(*self.knots.last().unwrap());
                }
            }
            Move::Left(n) => {
                for _ in 0..*n {
                    let mut head = self.knots.get_mut(0).unwrap();
                    head.0 -= 1;
                    for idx in 0..self.knots.len() - 1 {
                        let head = *self.knots.get(idx).unwrap();
                        let tail = self.knots.get_mut(idx + 1).unwrap();
                        move_tail(&head, tail);
                    }
                    self.tail_history.insert(*self.knots.last().unwrap());
                }
            }
            Move::Right(n) => {
                for _ in 0..*n {
                    let mut head = self.knots.get_mut(0).unwrap();
                    head.0 += 1;
                    for idx in 0..self.knots.len() - 1 {
                        let head = *self.knots.get(idx).unwrap();
                        let tail = self.knots.get_mut(idx + 1).unwrap();
                        move_tail(&head, tail);
                    }
                    self.tail_history.insert(*self.knots.last().unwrap());
                }
            }
        }
    }
}

/// Returns the two-dimensional distance between two knots.
fn distance(head: Knot, tail: Knot) -> (i16, i16) {
    (head.0 - tail.0, head.1 - tail.1)
}

/// Moves tail such that it is in a legal position relative to head.
fn move_tail(head: &Knot, tail: &mut Knot) {
    let (x, y) = distance(*head, *tail);
    if x.abs() + y.abs() > 2 {
        // Off in more than one direction, need to move diagnoally.
        tail.0 += if x > 0 {
            (x - 1).max(1)
        } else {
            (x + 1).min(-1)
        };
        tail.1 += if y > 0 {
            (y - 1).max(1)
        } else {
            (y + 1).min(-1)
        };
    } else if x > 0 {
        tail.0 += x - 1;
    } else if x < 0 {
        tail.0 += x + 1;
    } else if y > 0 {
        tail.1 += y - 1;
    } else if y < 0 {
        tail.1 += y + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(input), 13)
    }

    #[test]
    fn part2_example() {
        let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(input), 36)
    }
}
