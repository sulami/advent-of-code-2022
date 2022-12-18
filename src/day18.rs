use fxhash::FxHashSet;
use nom::{
    bytes::complete::tag,
    character::complete::u32,
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

pub fn solve() {
    let input = include_str!("../inputs/18.txt");
    println!("day 18-1: {}", part1(input));
    // println!("day 18-2: {}", part2(input));
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
    let connections: usize = rocks
        .par_iter()
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
            .filter(|coords| rocks.contains(coords))
            .count()
        })
        .sum();
    rocks.len() * 6 - connections
}

type Rock = [usize; 3];

fn parse_rock(i: &str) -> IResult<&str, Rock> {
    map(separated_list1(tag(","), u32), |coords| {
        [coords[0] as usize, coords[1] as usize, coords[2] as usize]
    })(i)
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
}
