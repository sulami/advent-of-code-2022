use std::cmp::Ordering;
use std::collections::BinaryHeap;

use fxhash::FxHashSet;

pub fn solve() {
    let input = include_str!("../inputs/24.txt");
    println!("day 24-1: {}", part1(input));
    println!("day 24-2: {}", part2(input));
}

fn part1(input: &str) -> usize {
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
    let mut blizzard_positions: Vec<[u128; 4]> =
        (0..height * width).map(|_| [0, 0, 0, 0]).collect();
    for blizzard in blizzards {
        (0..128 * 4).for_each(|time| {
            let (x, y) = blizzard.position_at(time, height, width);
            let idx = y * width + x;
            let bitmask = time / 128;
            let offset = time % 128;
            blizzard_positions[idx][bitmask] |= 1 << offset;
        });
    }

    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();

    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: start_x,
        y: -1,
        time: 0,
        reverse: false,
    }]);

    while let Some(state) = queue.pop() {
        if state.x == end_x && state.y == height as isize - 1 {
            return state.time;
        }

        if visited.contains(&(state.time, (state.x, state.y))) {
            continue;
        } else {
            visited.insert((state.time, (state.x, state.y)));
        }

        [
            (state.x, (state.y + 1).min(height as isize - 1)),
            ((state.x + 1).min(width - 1), state.y),
            (state.x, state.y),
            (state.x, (state.y - 1).max(0)),
            (state.x.saturating_sub(1), state.y),
        ]
        .iter()
        .copied()
        .filter(|(x, y)| *y >= 0 || *x == start_x)
        .filter(|(x, y)| {
            *y < 0
                || 0 == (blizzard_positions[*y as usize * width + x][state.time / 128]
                    & 1 << (state.time % 128))
        })
        .for_each(|opt| {
            let mut new_state = state;
            new_state.time += 1;
            (new_state.x, new_state.y) = opt;
            queue.push(new_state);
        });
    }

    panic!("failed to find a valid path")
}

fn part2(input: &str) -> usize {
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

    let mut trip = 0;

    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: start_x,
        y: -1,
        time: trip,
        reverse: false,
    }]);

    // Leg 1
    while let Some(state) = queue.pop() {
        if state.x == end_x && state.y == height as isize - 1 {
            trip += state.time - trip;
            break;
        }

        if visited.contains(&(state.time, (state.x, state.y))) {
            continue;
        } else {
            visited.insert((state.time, (state.x, state.y)));
        }

        [
            (state.x, (state.y + 1).min(height as isize - 1)),
            ((state.x + 1).min(width - 1), state.y),
            (state.x, state.y),
            (state.x, (state.y - 1).max(0)),
            (state.x.saturating_sub(1), state.y),
        ]
        .iter()
        .copied()
        .filter(|(x, y)| *y >= 0 || *x == start_x)
        .filter(|(x, y)| {
            *y < 0
                || 0 == (blizzard_positions[*y as usize * width + x][state.time / 128]
                    & 1 << (state.time % 128))
        })
        .for_each(|opt| {
            let mut new_state = state;
            new_state.time += 1;
            (new_state.x, new_state.y) = opt;
            queue.push(new_state);
        });
    }

    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: end_x,
        y: height as isize,
        time: trip,
        reverse: true,
    }]);

    // Leg 2 - back
    while let Some(state) = queue.pop() {
        if state.x == start_x && state.y == 0 {
            trip += state.time - trip;
            break;
        }

        if visited.contains(&(state.time, (state.x, state.y))) {
            continue;
        } else {
            visited.insert((state.time, (state.x, state.y)));
        }

        [
            (state.x, (state.y + 1).min(height as isize - 1)),
            ((state.x + 1).min(width - 1), state.y),
            (state.x, state.y),
            (state.x, (state.y - 1).max(0)),
            (state.x.saturating_sub(1), state.y),
        ]
        .iter()
        .copied()
        .filter(|(x, y)| *y < height as isize || *x == end_x)
        .filter(|(x, y)| {
            *y == height as isize
                || 0 == (blizzard_positions[*y as usize * width + x][state.time / 128]
                    & 1 << (state.time % 128))
        })
        .for_each(|opt| {
            let mut new_state = state;
            new_state.time += 1;
            (new_state.x, new_state.y) = opt;
            queue.push(new_state);
        });
    }

    let mut visited: FxHashSet<(usize, (usize, isize))> = FxHashSet::default();
    let mut queue: BinaryHeap<State> = BinaryHeap::from([State {
        x: start_x,
        y: -1,
        time: trip,
        reverse: false,
    }]);

    // Leg 3
    while let Some(state) = queue.pop() {
        if state.x == end_x && state.y == height as isize - 1 {
            trip += state.time - trip;
            break;
        }

        if visited.contains(&(state.time, (state.x, state.y))) {
            continue;
        } else {
            visited.insert((state.time, (state.x, state.y)));
        }

        [
            (state.x, (state.y + 1).min(height as isize - 1)),
            ((state.x + 1).min(width - 1), state.y),
            (state.x, state.y),
            (state.x, (state.y - 1).max(0)),
            (state.x.saturating_sub(1), state.y),
        ]
        .iter()
        .copied()
        .filter(|(x, y)| *y >= 0 || *x == start_x)
        .filter(|(x, y)| {
            *y < 0
                || 0 == (blizzard_positions[*y as usize * width + x][state.time / 128]
                    & 1 << (state.time % 128))
        })
        .for_each(|opt| {
            let mut new_state = state;
            new_state.time += 1;
            (new_state.x, new_state.y) = opt;
            queue.push(new_state);
        });
    }

    trip
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    x: usize,
    y: isize,
    time: usize,
    reverse: bool,
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
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 54);
    }
}
