use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../inputs/12.txt");
    println!("day 12-1: {}", part1(input));
    println!("day 12-2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let map: Map = input.parse().expect("invalid map");
    let mut walker = Walker::new(&map, map.start);
    walker.walk().expect("no route found")
}

fn part2(input: &str) -> usize {
    let map: Map = input.parse().expect("invalid map");
    (0..map.inner.len())
        .filter_map(|idx| {
            if map.inner[idx] == u32::from('a') {
                let mut walker = Walker::new(&map, idx);
                return walker.walk();
            }
            None
        })
        .min()
        .expect("no route found")
}

#[derive(Debug)]
struct Walker<'a> {
    map: &'a Map,
    candidates: VecDeque<Vec<usize>>,
    best: HashMap<usize, usize>,
}

impl<'a> Walker<'a> {
    fn new(map: &'a Map, start: usize) -> Self {
        let mut candidates = VecDeque::new();
        map.options(start)
            .iter()
            .for_each(|&o| candidates.push_front(vec![o]));
        let best = HashMap::new();
        Self {
            map,
            candidates,
            best,
        }
    }

    /// Breadth-first walk all possible paths, returning the distance
    /// once the first one finds the end.
    fn walk(&mut self) -> Option<usize> {
        while let Some(path) = self.candidates.pop_back() {
            let position = *path.last().unwrap();
            if self.map.inner[position] == 'E'.into() {
                return Some(path.len());
            }
            if let Some(record) = self.best.get(&position) {
                if record <= &path.len() {
                    continue;
                } else {
                    self.best.insert(position, path.len());
                }
            } else {
                self.best.insert(position, path.len());
            }
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
    start: usize,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s
            .chars()
            .position(|c| c == 'S')
            .ok_or("unable to find start")?;
        Ok(Map {
            inner: s
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.into())
                .collect(),
            width: s.lines().nth(0).ok_or("zero width map")?.chars().count(),
            start,
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
                Some(_) if this == 'S'.into() => true,
                Some(pos) => self
                    .inner
                    .get(pos as usize)
                    .map(|&other| {
                        if other == 'S'.into() {
                            return false;
                        }
                        if other == 'E'.into() {
                            return this >= 'y'.into();
                        }
                        return other <= this + 1;
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
