use std::collections::VecDeque;

use fxhash::FxHashSet;
use nom::{
    bytes::complete::tag,
    character::complete::i32,
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult,
};

pub fn solve() -> String {
    let input = include_str!("../inputs/18.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> usize {
    let rocks: FxHashSet<Rock> = input
        .lines()
        .map(|l| {
            all_consuming(parse_rock)(l)
                .expect("failed to parse rock")
                .1
        })
        .collect();

    // Get all rock-neighbouring fields that aren't rocks themselves,
    // thus sides of a rock that are touching the air.
    rocks
        .iter()
        .map(|[x, y, z]| {
            [
                [(*x + 1), *y, *z],
                [(*x - 1), *y, *z],
                [*x, (*y + 1), *z],
                [*x, (*y - 1), *z],
                [*x, *y, (*z + 1)],
                [*x, *y, (*z - 1)],
            ]
            .iter()
            .copied()
            .filter(|coords| !rocks.contains(coords))
            .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let rocks: FxHashSet<Rock> = input
        .lines()
        .map(|l| {
            all_consuming(parse_rock)(l)
                .expect("failed to parse rock")
                .1
        })
        .collect();

    // Find the bounds of the 3D shape described.
    let min_x = rocks.iter().map(|[x, _, _]| x).min().unwrap();
    let min_y = rocks.iter().map(|[_, y, _]| y).min().unwrap();
    let min_z = rocks.iter().map(|[_, _, z]| z).min().unwrap();
    let max_x = rocks.iter().map(|[x, _, _]| x).max().unwrap();
    let max_y = rocks.iter().map(|[_, y, _]| y).max().unwrap();
    let max_z = rocks.iter().map(|[_, _, z]| z).max().unwrap();

    // Collect all empty (non-rock) fields in the box surrounding the
    // shape.
    let empty_fields: FxHashSet<Rock> = (min_x - 1..=max_x + 1)
        .flat_map(|x| {
            (min_y - 1..=max_y + 1)
                .flat_map(|y| {
                    (min_z - 1..=max_z + 1)
                        .map(|z| [x, y, z])
                        .filter(|r| !rocks.contains(r))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // Flood fill from the outside to find all the empty fields that
    // are reachable by water.
    let outside = flood_fill([*min_x, *min_y, *min_z], &empty_fields);

    // Same as part 1, but limited to those reachable fields.
    rocks
        .iter()
        .map(|[x, y, z]| {
            [
                [(*x + 1), *y, *z],
                [(*x - 1), *y, *z],
                [*x, (*y + 1), *z],
                [*x, (*y - 1), *z],
                [*x, *y, (*z + 1)],
                [*x, *y, (*z - 1)],
            ]
            .iter()
            .copied()
            .filter(|coords| outside.contains(coords))
            .count()
        })
        .sum()
}

type Rock = [isize; 3];

fn parse_rock(i: &str) -> IResult<&str, Rock> {
    map(separated_list1(tag(","), i32), |coords| {
        [coords[0] as isize, coords[1] as isize, coords[2] as isize]
    })(i)
}

/// 3D flood-fill from the start, using only coordinates that are in
/// valid for traversal, returning the set of visited fields.
fn flood_fill(start: Rock, valid: &FxHashSet<Rock>) -> FxHashSet<Rock> {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::from([start]);
    while let Some([x, y, z]) = queue.pop_front() {
        let neighbours = [
            [(x + 1), y, z],
            [(x - 1), y, z],
            [x, (y + 1), z],
            [x, (y - 1), z],
            [x, y, (z + 1)],
            [x, y, (z - 1)],
        ];
        for n in &neighbours {
            if visited.contains(n) {
                continue;
            }
            if valid.contains(n) {
                visited.insert(*n);
                queue.push_back(*n);
            }
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 64);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 58);
    }
}
