use std::cmp::Ordering;
use std::collections::BinaryHeap;

use fxhash::FxHashSet;

pub fn solve() -> String {
    let input = include_str!("../inputs/24.txt");
    let (part1, part2) = both_parts(input);
    format!("{part1}\n{part2}")
}

fn both_parts(input: &str) -> (usize, usize) {
    let (height, width, start_x, end_x, blizzard_positions) = setup(input);
    let mut trip = 0;

    // Leg 1
    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: start_x,
        y: -1,
        time: trip,
        reverse: false,
    }]);
    while let Some(state) = queue.pop() {
        if let Some(distance) = state.step(
            height,
            width,
            start_x,
            end_x,
            &mut queue,
            &mut visited,
            &blizzard_positions,
        ) {
            trip = distance;
            break;
        }
    }

    let part1 = trip;

    // Leg 2 - back
    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: end_x,
        y: height as isize,
        time: trip,
        reverse: true,
    }]);
    while let Some(state) = queue.pop() {
        if let Some(distance) = state.step(
            height,
            width,
            start_x,
            end_x,
            &mut queue,
            &mut visited,
            &blizzard_positions,
        ) {
            trip = distance;
            break;
        }
    }

    // Leg 3 - there again
    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: start_x,
        y: -1,
        time: trip,
        reverse: false,
    }]);
    while let Some(state) = queue.pop() {
        if let Some(distance) = state.step(
            height,
            width,
            start_x,
            end_x,
            &mut queue,
            &mut visited,
            &blizzard_positions,
        ) {
            trip = distance;
            break;
        }
    }

    (part1, trip)
}

/// Parses the input and returns:
/// - the height and width of the usable map
/// - the x-coordinate of the start and end locations
/// - a Vec of bitmasks for each cell, discribing if there will be a storm
fn setup(input: &str) -> (usize, usize, usize, usize, Vec<[u128; 8]>) {
    let height = input.lines().count() - 2;
    let width = input.lines().next().unwrap().chars().count() - 2;
    let start_x = input.chars().skip(1).position(|c| c != '#').unwrap();
    let end_x = width - input.chars().rev().skip(1).position(|c| c != '#').unwrap();
    let blizzards: Vec<Blizzard> = input
        .lines()
        .skip(1)
        .take(height)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .skip(1)
                .take(width)
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '.' {
                        return None;
                    }
                    let heading = match c {
                        '>' => Heading::Right,
                        '<' => Heading::Left,
                        '^' => Heading::Up,
                        'v' => Heading::Down,
                        _ => panic!("invalid tile"),
                    };
                    Some(Blizzard { x, y, heading })
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut blizzard_positions: Vec<[u128; 8]> = (0..height * width)
        .map(|_| [0, 0, 0, 0, 0, 0, 0, 0])
        .collect();
    for blizzard in blizzards {
        (0..128 * 8).for_each(|time| {
            let (x, y) = blizzard.position_at(time, height, width);
            let idx = y * width + x;
            let bitmask = time / 128;
            let offset = time % 128;
            blizzard_positions[idx][bitmask] |= 1 << offset;
        });
    }
    (height, width, start_x, end_x, blizzard_positions)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    x: usize,
    y: isize,
    time: usize,
    reverse: bool,
}

impl State {
    #[allow(clippy::too_many_arguments)]
    /// Takes a step during BFS pathfinding, returns `Some(distance)`
    /// if it reaches the goal.
    fn step(
        &self,
        height: usize,
        width: usize,
        start_x: usize,
        end_x: usize,
        queue: &mut BinaryHeap<Self>,
        visited: &mut FxHashSet<(usize, (usize, isize))>,
        blizzard_positions: &[[u128; 8]],
    ) -> Option<usize> {
        if (!self.reverse && self.x == end_x && self.y == height as isize - 1)
            || (self.reverse && self.x == start_x && self.y == 0)
        {
            return Some(self.time);
        }

        if visited.contains(&(self.time, (self.x, self.y))) {
            return None;
        } else {
            visited.insert((self.time, (self.x, self.y)));
        }

        [
            (self.x, (self.y + 1).min(height as isize - 1)),
            ((self.x + 1).min(width - 1), self.y),
            (self.x, self.y),
            (self.x, (self.y - 1).max(0)),
            (self.x.saturating_sub(1), self.y),
        ]
        .iter()
        .copied()
        .filter(|(x, y)| {
            // Allow waiting outside the grid in the starting position.
            (!self.reverse && (*y >= 0 || *x == start_x))
                || (self.reverse && (*y < height as isize || *x == end_x))
        })
        .filter(|(x, y)| {
            (*y < 0 && !self.reverse)
                || (*y == height as isize && self.reverse)
                || 0 == (blizzard_positions[*y as usize * width + x][self.time / 128]
                    & 1 << (self.time % 128))
        })
        .for_each(|opt| {
            let mut new_self = *self;
            new_self.time += 1;
            (new_self.x, new_self.y) = opt;
            queue.push(new_self);
        });
        None
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.reverse {
            other
                .time
                .cmp(&self.time)
                .then_with(|| (other.x as isize + other.y).cmp(&(self.x as isize + self.y)))
        } else {
            other
                .time
                .cmp(&self.time)
                .then_with(|| (self.x as isize + self.y).cmp(&(other.x as isize + other.y)))
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    heading: Heading,
}

#[derive(Copy, Clone, Debug)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Blizzard {
    /// Returns the position of this blizzard at a given time.
    fn position_at(&self, time: usize, height: usize, width: usize) -> (usize, usize) {
        match self.heading {
            Heading::Left => (((time / width + 1) * width + self.x - time) % width, self.y),
            Heading::Right => ((self.x + time) % width, self.y),
            Heading::Up => (
                self.x,
                ((time / height + 1) * height + self.y - time) % height,
            ),
            Heading::Down => (self.x, (self.y + time) % height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.# ";

    #[test]
    fn part1_example() {
        assert_eq!(both_parts(INPUT).0, 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(both_parts(INPUT).1, 54);
    }
}
