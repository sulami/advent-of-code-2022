use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn solve() -> String {
    let input = include_str!("../inputs/23.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> usize {
    let mut elves = parse_elves(input);

    // Three relative coordinate pairs to check, and a relative
    // coordinate pair to propose if that check succeeds.
    let mut preferences = [
        // North
        ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
        // South
        ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
        // West
        ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
        // East
        ([(1, -1), (1, 0), (1, 1)], (1, 0)),
    ];

    for _ in 0..10 {
        // Tracks how many elves propose to go to a given tile.
        let mut proposal_counts: FxHashMap<Tile, usize> = FxHashMap::default();
        // Create a hash set of elf positions for faster checking of
        // neighbours.
        let taken: FxHashSet<&Elf> = elves.iter().collect();
        // Gather proposals.
        let proposals: Vec<Option<Tile>> = elves
            .iter()
            .map(|e| propose_move(e, &preferences, &taken, &mut proposal_counts))
            .collect();
        // Execute proposed moves.
        elves
            .iter_mut()
            .zip(proposals)
            .for_each(|(elf, proposal)| execute_move(elf, proposal, &proposal_counts));
        // Rotate movement direction preferences.
        preferences.rotate_left(1);
    }

    let (box_width, box_height) = bounding_box(&elves);
    box_width * box_height - elves.len()
}

fn part2(input: &str) -> usize {
    let mut elves = parse_elves(input);

    // Three relative coordinate pairs to check, and a relative
    // coordinate pair to propose if that check succeeds.
    let mut preferences = [
        // North
        ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
        // South
        ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
        // West
        ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
        // East
        ([(1, -1), (1, 0), (1, 1)], (1, 0)),
    ];

    let mut i = 0;

    loop {
        i += 1;
        // Tracks how many elves propose to go to a given tile.
        let mut proposal_counts: FxHashMap<Tile, usize> = FxHashMap::default();
        // Create a hash set of elf positions for faster checking of
        // neighbours.
        let taken: FxHashSet<&Elf> = elves.iter().collect();
        // Gather proposals.
        let proposals: Vec<Option<Tile>> = elves
            .iter()
            .map(|e| propose_move(e, &preferences, &taken, &mut proposal_counts))
            .collect();
        if proposals.iter().all(Option::is_none) {
            break;
        }
        // Execute proposed moves.
        elves
            .iter_mut()
            .zip(proposals)
            .for_each(|(elf, proposal)| execute_move(elf, proposal, &proposal_counts));
        // Rotate movement direction preferences.
        preferences.rotate_left(1);
    }

    i
}

fn parse_elves(input: &str) -> Vec<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(|(linum, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(colnum, c)| {
                    if c == '#' {
                        Some((colnum as isize, linum as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// (x, y) coordinate pair, where x is right, and y is down. The
/// origin happens to be at the top left corner of the input, but is
/// irrelevant because we use signed coordinates.
type Elf = (isize, isize);
type Tile = (isize, isize);

/// Execute a proposed move for this elf, if this is the only elf that
/// proposes to make this move, otherwise do nothing.
fn execute_move(elf: &mut Elf, proposal: Option<Tile>, counts: &FxHashMap<Tile, usize>) {
    if let Some(p) = proposal {
        if *counts.get(&p).unwrap() == 1 {
            *elf = p;
        }
    }
}

/// Propose a move for this elf, and also record it in counts. Can
/// propose nothing if there are no elves around, or there are no good
/// options.
fn propose_move(
    elf: &Elf,
    preferences: &[([Tile; 3], Tile)],
    elves: &FxHashSet<&Elf>,
    counts: &mut FxHashMap<Tile, usize>,
) -> Option<Tile> {
    let tile_free = |&(x, y)| !elves.contains(&(elf.0 + x, elf.1 + y));
    let mut alone = true;
    for neighbour in &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        if !tile_free(neighbour) {
            alone = false;
            break;
        }
    }
    if alone {
        return None;
    }

    let mut proposal = None;

    for ([a, b, c], (x, y)) in preferences {
        if tile_free(a) && tile_free(b) && tile_free(c) {
            proposal = Some((elf.0 + x, elf.1 + y));
            break;
        }
    }

    if let Some(prop) = proposal {
        if let Some(p) = counts.get_mut(&prop) {
            *p += 1;
        } else {
            counts.insert(prop, 1);
        };
    };
    proposal
}

/// Returns the size of the smallest rectangle that contains the
/// elves.
fn bounding_box(elves: &[Elf]) -> (usize, usize) {
    let (min_x, max_x) = elves.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = elves.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (1 + min_x.abs_diff(*max_x), 1 + min_y.abs_diff(*max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 110);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 20);
    }
}
