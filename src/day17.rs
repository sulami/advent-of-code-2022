use std::collections::VecDeque;

pub fn solve() {
    let input = include_str!("../inputs/17.txt");
    println!("day 17-1: {}", part1(input));
    println!("day 17-2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let jets = parse_jets(input);
    let mut jets = jets.iter().copied().cycle();
    let mut chamber = Chamber::new(&mut jets);
    (0..2022).for_each(|_| chamber.spawn_piece());
    chamber.real_height()
}

fn part2(input: &str) -> u128 {
    let jets = parse_jets(input);
    let mut infinite_jets = jets.iter().copied().cycle();
    let mut chamber = Chamber::new(&mut infinite_jets);

    // 1T is way too many iterations. But because inputs are
    // deterministic, we know it can loop. This records a pattern of 5
    // heights, which repeats eventually. With that we can extrapolate
    // total height gain.
    let input_repetition_after = jets.len() as u128 * 5;
    let mut repetition_pattern: Vec<usize> = vec![];

    for i in 0..input_repetition_after * 5 {
        if i > 0 && i % input_repetition_after == 0 {
            repetition_pattern.push(chamber.height());
        }
        chamber.spawn_piece()
    }
    let base_height = chamber.real_height() as u128;

    let mut heights: VecDeque<usize> = VecDeque::default();
    let mut loop_iterations = 0u128;

    for i in 0..1_000_000_000_000u128 {
        if i > 0 && i % input_repetition_after as u128 == 0 {
            heights.push_back(chamber.height());
            if heights.len() > 5 {
                let _ = heights.pop_front();
            }
            if heights
                .iter()
                .zip(repetition_pattern.iter())
                .all(|(a, b)| *a == *b)
            {
                loop_iterations = i;
                break;
            }
        }
        chamber.spawn_piece()
    }

    let post_loop_height = chamber.real_height() as u128;
    let loop_gain = chamber.real_height() as u128 - base_height;
    let loops = (1_000_000_000_000u128 - input_repetition_after * 5) / loop_iterations;
    let extra_iterations = (1_000_000_000_000u128 - input_repetition_after * 5) % loop_iterations;

    for _ in 0..extra_iterations {
        chamber.spawn_piece();
    }

    let extra_height = chamber.real_height() as u128 - post_loop_height;

    base_height + loops * loop_gain + extra_height
}

struct Chamber<'a> {
    source: TetrisPieceSource,
    jets: &'a mut dyn Iterator<Item = Jet>,
    /// Each row is the lower 7 bits, where a set bit is a piece. The
    /// first row is the bottom-most.
    inner: Vec<u8>,
    /// The number of rows that is unreachable for newly falling
    /// pieces. Because we only care about tower height, we can remove
    /// those rows, almost like real Tetris.
    unreachable: usize,
}

impl<'a> Chamber<'a> {
    fn new(jets: &'a mut impl Iterator<Item = Jet>) -> Self {
        Self {
            source: TetrisPieceSource::default(),
            jets,
            inner: Vec::default(),
            unreachable: 0,
        }
    }

    /// Returns the height of the tower built, including unreachable
    /// rows.
    fn real_height(&self) -> usize {
        self.unreachable + self.height()
    }

    /// Returns the height of the tower built, excluding unreachable
    /// rows. Useful for indexing into the inner Vec.
    fn height(&self) -> usize {
        self.inner.len() - self.inner.iter().rev().take_while(|&r| *r == 0).count()
    }

    /// Spans the next piece and simulates it falling until it comes
    /// to rest somewhere, modifying the internal state.
    fn spawn_piece(&mut self) {
        let piece = self.source.next().unwrap();
        let mut x: usize = 2;
        let mut y: usize = self.height() + 3;

        // Ensure we have a healthy padding of empty rows at the top.
        if self.inner.iter().rev().take_while(|&r| *r == 0).count() < 6 {
            self.inner.extend([0, 0, 0, 0, 0, 0]);
        }

        loop {
            // Apply the jet.
            let proposed_x = match self.jets.next() {
                Some(Jet::Left) => x.saturating_sub(1),
                Some(Jet::Right) => (7 - piece.width()).min(x + 1),
                None => unreachable!(),
            };
            if proposed_x != x && !piece.collides(proposed_x, y, &self.inner) {
                x = proposed_x;
            }

            // Try to fall down.
            if y == 0 || piece.collides(x, y - 1, &self.inner) {
                break;
            } else {
                y -= 1;
            }
        }

        // Apply the piece bits to existing rows where the piece comes
        // to rest.
        for (i, p) in piece.binary_repr().iter().enumerate() {
            self.inner[y + i] |= p >> x;
        }

        // If any rows became unreachable, chop them off.
        if let Some(idx) = self.inner.iter().rposition(|row| *row == 0b01111111) {
            self.unreachable += idx + 1;
            self.inner = self.inner.split_off(idx + 1);
            // Reserve some extra space to avoid many small allocations.
            self.inner.reserve(64);
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Jet {
    Left,
    Right,
}

fn parse_jets(s: &str) -> Vec<Jet> {
    s.trim()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("invalid jet"),
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
enum TetrisPiece {
    HorizontalBar,
    Plus,
    LShape,
    VerticalBar,
    Box,
}

impl TetrisPiece {
    /// Returns the width from the left edge.
    fn width(&self) -> usize {
        match self {
            Self::HorizontalBar => 4,
            Self::Plus => 3,
            Self::LShape => 3,
            Self::VerticalBar => 1,
            Self::Box => 2,
        }
    }

    /// Returns a slice of rows that represent this piece. The rows
    /// are in bottom-top order. The piece is aligned at x = 0.
    fn binary_repr(&self) -> [u8; 4] {
        match self {
            Self::HorizontalBar => [0b01111000, 0, 0, 0],
            Self::Plus => [0b00100000, 0b01110000, 0b00100000, 0],
            Self::LShape => [0b01110000, 0b00010000, 0b00010000, 0],
            Self::VerticalBar => [0b01000000, 0b01000000, 0b01000000, 0b01000000],
            Self::Box => [0b01100000, 0b01100000, 0, 0],
        }
    }

    /// Returns true if the piece would collide with an existing piece
    /// in the chamber if placed at the given position.
    fn collides(&self, x: usize, y: usize, chamber: &[u8]) -> bool {
        chamber
            .iter()
            .skip(y)
            .zip(self.binary_repr())
            .any(|(existing, piece)| (piece >> x) & existing != 0)
    }
}

#[derive(Debug)]
struct TetrisPieceSource {
    last: TetrisPiece,
}

impl Default for TetrisPieceSource {
    fn default() -> Self {
        Self {
            last: TetrisPiece::Box,
        }
    }
}

impl Iterator for TetrisPieceSource {
    type Item = TetrisPiece;

    fn next(&mut self) -> Option<Self::Item> {
        let piece = match self.last {
            TetrisPiece::HorizontalBar => TetrisPiece::Plus,
            TetrisPiece::Plus => TetrisPiece::LShape,
            TetrisPiece::LShape => TetrisPiece::VerticalBar,
            TetrisPiece::VerticalBar => TetrisPiece::Box,
            TetrisPiece::Box => TetrisPiece::HorizontalBar,
        };
        self.last = piece;
        Some(piece)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3068);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
