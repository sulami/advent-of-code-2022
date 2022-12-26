use nom::{
    bytes::complete::tag,
    character::complete::u16,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

/// Cave depth needs to be at least the lowest rock y-position, plus
/// some padding for part 2. In my case that is somewhere around 170,
/// but YMMV.
const CAVE_DEPTH: usize = 255;

pub fn solve() -> String {
    let input = include_str!("../inputs/14.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> usize {
    let mut cave = Cave::new(parse_paths(input));
    cave.drop_sand_until_terminal()
}

fn part2(input: &str) -> usize {
    let mut paths = parse_paths(input);
    let floor = 2 + paths
        .iter()
        .map(|path| path.iter().map(|point| point.y).max().unwrap())
        .max()
        .unwrap();
    paths.push(vec![Point { x: 0, y: floor }, Point { x: 1000, y: floor }]);
    let mut cave = Cave::new(paths);
    cave.drop_sand_until_terminal()
}

#[derive(Debug)]
struct Cave {
    inner: [Cell; 1000 * CAVE_DEPTH],
}

impl Cave {
    /// Returns a newly constructed cave with rock as indicated by
    /// paths.
    fn new(paths: Vec<Path>) -> Self {
        let mut inner = [Cell::Empty; 1000 * CAVE_DEPTH];
        for path in paths {
            for i in 0..path.len() - 1 {
                let start = &path[i];
                let end = &path[i + 1];
                if start.x == end.x {
                    // Vertical section
                    for y in start.y.min(end.y)..=start.y.max(end.y) {
                        inner[idx(start.x, y)] = Cell::Rock;
                    }
                } else {
                    // Horizontal section
                    for x in start.x.min(end.x)..=start.x.max(end.x) {
                        inner[idx(x, start.y)] = Cell::Rock;
                    }
                }
            }
        }
        Self { inner }
    }

    /// Keeps dropping sand into the cave until it either falls out
    /// the bottom, or starts blocking the entrance. Returns the
    /// number of sand units dropped.
    fn drop_sand_until_terminal(&mut self) -> usize {
        let mut counter = 0;
        while !self.drop_one_unit_of_sand() {
            counter += 1;
        }
        counter
    }

    /// Drops one sand unit into the cave. Returns true if the sand fell
    /// through into the void below, or came to rest in front of the
    /// inlet.
    fn drop_one_unit_of_sand(&mut self) -> bool {
        let (mut x, mut y) = (500, 0);
        loop {
            // Fell through.
            if y >= CAVE_DEPTH - 1 {
                return true;
            }
            // We're overflowing.
            if self.inner[idx(x, y)] == Cell::Sand {
                return true;
            }
            // Try falling straight down.
            if self.inner[idx(x, y + 1)] == Cell::Empty {
                y += 1;
                continue;
            }
            // Try falling to the left.
            if self.inner[idx(x - 1, y + 1)] == Cell::Empty {
                x -= 1;
                y += 1;
                continue;
            }
            // Try falling to the right.
            if self.inner[idx(x + 1, y + 1)] == Cell::Empty {
                x += 1;
                y += 1;
                continue;
            }
            // Settled.
            self.inner[idx(x, y)] = Cell::Sand;
            return false;
        }
    }
}

/// Returns the cave index for a given x, y coordinate pair.
fn idx(x: usize, y: usize) -> usize {
    y * 1000 + x
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

type Path = Vec<Point>;

fn parse_paths(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|l| {
            all_consuming(parse_path)(l)
                .expect("failed to parse path")
                .1
        })
        .collect()
}

fn parse_path(i: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), parse_point)(i)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_point(i: &str) -> IResult<&str, Point> {
    map(tuple((u16, tag(","), u16)), |(x, _, y)| Point {
        x: x as usize,
        y: y as usize,
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 24);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 93);
    }
}
