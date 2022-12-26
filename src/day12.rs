/// This solves backwards, walking the map from 'E' to 'S'/'a', to
/// make part 2 much faster.
use std::collections::VecDeque;
use std::str::FromStr;

use fxhash::FxHashSet;

pub fn solve() -> String {
    let input = include_str!("../inputs/12.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> usize {
    let map: Map = input.parse().expect("invalid map");
    let mut walker = Walker::new(&map, 'S'.into());
    walker.walk().expect("no route found")
}

fn part2(input: &str) -> usize {
    let map: Map = input.parse().expect("invalid map");
    let mut walker = Walker::new(&map, 'a'.into());
    walker.walk().expect("no route found")
}

#[derive(Debug)]
struct Walker<'a> {
    goal: u32,
    map: &'a Map,
    candidates: VecDeque<Vec<usize>>,
    visited: FxHashSet<usize>,
}

impl<'a> Walker<'a> {
    fn new(map: &'a Map, goal: u32) -> Self {
        let mut candidates = VecDeque::new();
        map.options(map.end)
            .iter()
            .for_each(|&o| candidates.push_front(vec![o]));
        let visited = FxHashSet::default();
        Self {
            goal,
            map,
            candidates,
            visited,
        }
    }

    /// Breadth-first walk all possible paths, returning the distance
    /// once the first one finds the end.
    fn walk(&mut self) -> Option<usize> {
        while let Some(path) = self.candidates.pop_back() {
            let position = *path.last().unwrap();
            if self.map.inner[position] == self.goal {
                return Some(path.len());
            }
            if self.visited.contains(&position) {
                continue;
            }
            self.visited.insert(position);
            self.map
                .options(position)
                .iter()
                .filter(|&idx| !path.contains(idx))
                .for_each(|&o| {
                    let mut p = path.clone();
                    p.push(o);
                    self.candidates.push_front(p);
                });
        }
        None
    }
}

#[derive(Clone, Debug)]
struct Map {
    inner: Vec<u32>,
    width: usize,
    end: usize,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end = s
            .chars()
            .filter(|c| c.is_alphabetic())
            .position(|c| c == 'E')
            .ok_or("unable to find end")?;
        Ok(Map {
            inner: s
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.into())
                .collect(),
            width: s.lines().next().ok_or("zero width map")?.chars().count(),
            end,
        })
    }
}

impl Map {
    /// For a given position, return a vec of possible target
    /// positions.
    fn options(&self, idx: usize) -> Vec<usize> {
        let this = self.inner[idx];
        let mut rv = vec![];
        // Check a given target position for validity, based on the
        // current position plus offset.
        let check_idx = |offset: isize| -> bool {
            match (idx as isize).checked_add(offset) {
                None => false,
                Some(pos) if pos < 0 => false,
                Some(pos) if pos as usize >= self.inner.len() => false,
                Some(pos) => self
                    .inner
                    .get(pos as usize)
                    .map(|&other| {
                        if other == 'E'.into() {
                            return false;
                        }
                        if other == 'S'.into() {
                            return this <= 'b'.into();
                        }
                        if this == 'E'.into() {
                            return other >= 'y'.into();
                        }
                        this <= other + 1
                    })
                    .unwrap_or(false),
            }
        };
        // Left
        if check_idx(-1) {
            rv.push(idx - 1);
        }
        // Right
        if check_idx(1) {
            rv.push(idx + 1);
        }
        // Up
        if check_idx(-(self.width as isize)) {
            rv.push(idx - self.width);
        }
        // Down
        if check_idx(self.width as isize) {
            rv.push(idx + self.width);
        }
        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 29);
    }
}
