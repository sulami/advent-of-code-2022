use nom::{
    branch::alt,
    character::complete::{one_of, u8},
    combinator::{all_consuming, map},
    multi::many1,
    IResult,
};

pub fn solve() -> String {
    let input = include_str!("../inputs/22.txt");
    format!("{}", part1(input))
}

fn part1(input: &str) -> usize {
    let (map, instructions) = input.trim_end().split_once("\n\n").expect("invalid input");

    let instructions = all_consuming(many1(parse_instruction))(instructions)
        .expect("failed to parse instructions")
        .1;

    let mut map = Map::new(map);
    for instruction in instructions {
        map.execute(instruction);
    }
    1000 * (map.position / map.width + 1) + 4 * (map.position % map.width + 1) + map.facing.score()
}

struct Map {
    inner: Vec<Cell>,
    width: usize,
    position: usize,
    facing: Facing,
}

impl Map {
    fn new(i: &str) -> Self {
        let width = i.lines().map(str::len).max().unwrap();
        let map_rows: Vec<&str> = i.lines().collect();
        let inner: Vec<Cell> = map_rows
            .iter()
            .flat_map(|r| {
                let mut cells: Vec<Cell> = r
                    .chars()
                    .map(|c| match c {
                        ' ' => Cell::Void,
                        '.' => Cell::Empty,
                        '#' => Cell::Wall,
                        _ => panic!("invalid cell"),
                    })
                    .collect();
                // Pad the back of the row if it's too short.
                cells.extend((0..width - cells.len()).map(|_| Cell::Void));
                cells
            })
            .collect();
        let position = inner.iter().position(|c| *c == Cell::Empty).unwrap();
        let facing = Facing::Right;
        Self {
            inner,
            width,
            position,
            facing,
        }
    }

    /// Executes an instruction, either turning, or walking a given
    /// distance, stopping if we hit a wall, and wrapping if we walk
    /// off the map or into the void.
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Walk(dist) => {
                for _ in 0..dist {
                    if let Some(new) = self.next_cell() {
                        self.position = new;
                    } else {
                        break;
                    }
                }
            }
            _ => {
                self.facing = self.facing.turn(instruction);
            }
        }
    }

    /// Returns the index of the next step in a given facing. Returns None
    /// if blocked.
    fn next_cell(&self) -> Option<usize> {
        let step = |pos: usize| match self.facing {
            // Some weird typecasting going on here to make unsigned
            // modulo work when going left or up.
            Facing::Up => {
                (self.inner.len() as isize + pos as isize - self.width as isize) as usize
                    % self.inner.len()
            }
            Facing::Left => {
                pos / self.width * self.width
                    + ((pos as isize - 1) + self.width as isize) as usize % self.width
            }
            Facing::Down => (pos + self.width) % self.inner.len(),
            Facing::Right => pos / self.width * self.width + (pos + 1) % self.width,
        };
        let mut next = self.position;
        next = step(next);
        // Skip over the void by just continuing in the same
        // direction.
        while self.inner[next] == Cell::Void {
            next = step(next);
        }
        if self.inner[next] == Cell::Empty {
            Some(next)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Void,
    Empty,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Walk(u8),
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone, Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    /// Given an instruction, returns a new direction.
    fn turn(&self, instruction: Instruction) -> Self {
        match (self, instruction) {
            (Self::Up, Instruction::TurnLeft) => Self::Left,
            (Self::Left, Instruction::TurnLeft) => Self::Down,
            (Self::Down, Instruction::TurnLeft) => Self::Right,
            (Self::Right, Instruction::TurnLeft) => Self::Up,
            (Self::Up, Instruction::TurnRight) => Self::Right,
            (Self::Right, Instruction::TurnRight) => Self::Down,
            (Self::Down, Instruction::TurnRight) => Self::Left,
            (Self::Left, Instruction::TurnRight) => Self::Up,
            _ => *self,
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        map(u8, Instruction::Walk),
        map(one_of("LR"), |c| match c {
            'L' => Instruction::TurnLeft,
            'R' => Instruction::TurnRight,
            _ => unreachable!(),
        }),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    // NB This format is avoid the compiler munging the leading
    // whitespace.
    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 6032);
    }
}
