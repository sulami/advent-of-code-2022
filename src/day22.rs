use nom::{
    branch::alt,
    character::complete::{one_of, u8},
    combinator::{all_consuming, map},
    multi::many1,
    IResult,
};

pub fn solve() -> String {
    let input = include_str!("../inputs/22.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> usize {
    let (map, instructions) = input.trim_end().split_once("\n\n").expect("invalid input");

    let instructions = all_consuming(many1(parse_instruction))(instructions)
        .expect("failed to parse instructions")
        .1;

    let mut map = Map::new(map, false);
    for instruction in instructions {
        map.execute(instruction);
    }
    1000 * (map.position / map.width + 1) + 4 * (map.position % map.width + 1) + map.facing.score()
}

fn part2(input: &str) -> String {
    let (map, instructions) = input.trim_end().split_once("\n\n").expect("invalid input");

    let instructions = all_consuming(many1(parse_instruction))(instructions)
        .expect("failed to parse instructions")
        .1;

    let mut map = Map::new(map, true);
    for instruction in instructions {
        map.execute(instruction);
    }
    format!(
        "{}",
        1000 * (map.position / map.width + 1)
            + 4 * (map.position % map.width + 1)
            + map.facing.score()
    )
}

struct Map {
    inner: Vec<Cell>,
    width: usize,
    position: usize,
    facing: Facing,
    cubic: bool,
}

impl Map {
    fn new(i: &str, cubic: bool) -> Self {
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
            cubic,
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

    /// Returns a 3D position & facing for part 2. X, Y are 0 in the
    /// top-left corner of the front face, Z extends "into" the cube.
    /// The mapping here is hard-coded to my layout. Zero coordinates
    /// are considered on the outside of the cube, as are face_size +
    /// 1.
    fn position_3d(&self) -> ((usize, usize, usize), Facing3D) {
        let face_size = self.face_size();
        // Coordinates on face.
        let x = (self.position % self.width) % face_size;
        let y = (self.position / self.width) % face_size;
        let (face_x, face_y) = self.cube_face(self.position);
        match (face_x, face_y) {
            // Face A, front
            (1, 0) => ((x + 1, y + 1, 0), Facing3D::from(self.facing)),
            // Face B, bottom
            (1, 1) => (
                (x + 1, face_size + 1, y + 1),
                Facing3D::from(self.facing).rotate(Axis::X, 1),
            ),
            // Face C, back (upside down)
            (1, 2) => (
                (x + 1, face_size - y, face_size + 1),
                Facing3D::from(self.facing).rotate(Axis::X, 2),
            ),
            // Face D, top (turned)
            (0, 3) => (
                (y + 1, 0, x + 1),
                Facing3D::from(self.facing)
                    .rotate(Axis::X, 3)
                    .rotate(Axis::Y, 3),
            ),
            // Face E, left (upside down)
            (0, 2) => (
                (0, face_size - y, x + 1),
                Facing3D::from(self.facing)
                    .rotate(Axis::Y, 1)
                    .rotate(Axis::X, 2),
            ),
            // Face F, right
            (2, 0) => (
                (face_size + 1, y + 1, x + 1),
                Facing3D::from(self.facing).rotate(Axis::Y, 3),
            ),
            _ => unreachable!("unexpected cube face"),
        }
    }

    /// Given a 3D position & facing, returns the corresponding 2D
    /// coordinates and facing. Hard-coded to my layout.
    fn position_2d(&self, coords: (usize, usize, usize), facing: Facing3D) -> (usize, Facing) {
        let face_size = self.face_size();
        let (x, y, z) = coords;
        let (face_x, face_y) = match coords {
            // Face E, left
            (0, _, _) => (0, 2),
            // Face F, right
            (x, _, _) if x == face_size + 1 => (2, 0),
            // Face D, top
            (_, 0, _) => (0, 3),
            // Face B, bottom
            (_, y, _) if y == face_size + 1 => (1, 1),
            // Face A, front
            (_, _, 0) => (1, 0),
            // Face C, back
            (_, _, z) if z == face_size + 1 => (1, 2),
            _ => unreachable!("unexpeced 3d coordinates"),
        };
        let to_idx = |x_source, y_source| {
            // Face offset from top.
            face_y * face_size * self.width
            // Face offset from left.
                + face_x * face_size
                // Offset from face top.
                + y_source * self.width
                // Offset from face left.
                + x_source
        };
        match (face_x, face_y) {
            // Face A, front
            (1, 0) => (to_idx(x - 1, y - 1), facing.into()),
            // Face B, bottom
            (1, 1) => (to_idx(x - 1, z - 1), facing.rotate(Axis::X, 3).into()),
            // Face C, back (upside down)
            (1, 2) => (
                to_idx(x - 1, face_size - y),
                facing.rotate(Axis::X, 2).into(),
            ),
            // Face D, top (turned)
            (0, 3) => (
                to_idx(z - 1, x - 1),
                facing.rotate(Axis::Y, 1).rotate(Axis::X, 1).into(),
            ),
            // Face E, left (upside down)
            (0, 2) => (
                to_idx(z - 1, face_size - y),
                facing.rotate(Axis::X, 2).rotate(Axis::Y, 3).into(),
            ),
            // Face F, right
            (2, 0) => (to_idx(z - 1, y - 1), facing.rotate(Axis::Y, 1).into()),
            _ => unreachable!("unexpected face coordinates"),
        }
    }

    /// Returns the index of the next step in a given facing. Returns None
    /// if blocked.
    fn next_cell(&mut self) -> Option<usize> {
        if self.cubic {
            let face_size = self.face_size();
            let ((mut x, mut y, mut z), mut f3d) = self.position_3d();
            match f3d {
                Facing3D::Up => {
                    if y == 1 && z == 0 {
                        // Wrapping on face A.
                        f3d = Facing3D::In;
                        z += 1;
                    } else if y == 1 && z == face_size + 1 {
                        // Wrapping on face C.
                        f3d = Facing3D::Out;
                        z -= 1;
                    } else if y == 1 && x == 0 {
                        // Wrapping on face E.
                        f3d = Facing3D::Right;
                        x += 1;
                    } else if y == 1 && x == face_size + 1 {
                        // Wrapping on face F.
                        f3d = Facing3D::Left;
                        x -= 1;
                    } else if y == 1 {
                        panic!("unexpected wrapping position")
                    }
                    y -= 1;
                }
                Facing3D::Down => {
                    if y == face_size && z == 0 {
                        // Wrapping on face A.
                        f3d = Facing3D::In;
                        z += 1;
                    } else if y == face_size && z == face_size + 1 {
                        // Wrapping on face C.
                        f3d = Facing3D::Out;
                        z -= 1;
                    } else if y == face_size && x == 0 {
                        // Wrapping on face E.
                        f3d = Facing3D::Right;
                        x += 1;
                    } else if y == face_size && x == face_size + 1 {
                        // Wrapping on face F.
                        f3d = Facing3D::Left;
                        x -= 1;
                    } else if y == face_size {
                        panic!("unexpected wrapping position")
                    }
                    y += 1;
                }
                Facing3D::Left => {
                    if x == 1 && z == 0 {
                        // Wrapping on face A.
                        f3d = Facing3D::In;
                        z += 1;
                    } else if x == 1 && z == face_size + 1 {
                        // Wrapping on face C.
                        f3d = Facing3D::Out;
                        z -= 1;
                    } else if x == 1 && y == 0 {
                        // Wrapping on face D.
                        f3d = Facing3D::Down;
                        y += 1;
                    } else if x == 1 && y == face_size + 1 {
                        // Wrapping on face B.
                        f3d = Facing3D::Up;
                        y -= 1;
                    } else if x == 1 {
                        panic!("unexpected wrapping position: {x},{y},{z}");
                    }
                    x -= 1;
                }
                Facing3D::Right => {
                    if x == face_size && z == 0 {
                        // Wrapping on face A.
                        f3d = Facing3D::In;
                        z += 1;
                    } else if x == face_size && z == face_size + 1 {
                        // Wrapping on face C.
                        f3d = Facing3D::Out;
                        z -= 1;
                    } else if x == face_size && y == 0 {
                        // Wrapping on face D.
                        f3d = Facing3D::Down;
                        y += 1;
                    } else if x == face_size && y == face_size + 1 {
                        // Wrapping on face B.
                        f3d = Facing3D::Up;
                        y -= 1;
                    } else if x == face_size {
                        panic!("unexpected wrapping position: {x},{y},{z}");
                    }
                    x += 1;
                }
                Facing3D::In => {
                    if z == face_size && y == 0 {
                        // Wrapping on face D.
                        f3d = Facing3D::Down;
                        y += 1;
                    } else if z == face_size && y == face_size + 1 {
                        // Wrapping on face B.
                        f3d = Facing3D::Up;
                        y -= 1;
                    } else if z == face_size && x == 0 {
                        // Wrapping on face E.
                        f3d = Facing3D::Right;
                        x += 1;
                    } else if z == face_size && x == face_size + 1 {
                        // Wrapping on face F.
                        f3d = Facing3D::Left;
                        x -= 1;
                    } else if z == face_size {
                        panic!("unexpected wrapping position: {x},{y},{z}");
                    }
                    z += 1;
                }
                Facing3D::Out => {
                    if z == 1 && y == 0 {
                        // Wrapping on face D.
                        f3d = Facing3D::Down;
                        y += 1;
                    } else if z == 1 && y == face_size + 1 {
                        // Wrapping on face B.
                        f3d = Facing3D::Up;
                        y -= 1;
                    } else if z == 1 && x == 0 {
                        // Wrapping on face E.
                        f3d = Facing3D::Right;
                        x += 1;
                    } else if z == 1 && x == face_size + 1 {
                        // Wrapping on face F.
                        f3d = Facing3D::Left;
                        x -= 1;
                    } else if z == 1 {
                        panic!("unexpected wrapping position: {x},{y},{z}");
                    }
                    z -= 1;
                }
            };
            let (next, f2d) = self.position_2d((x, y, z), f3d);

            if self.inner[next] == Cell::Empty {
                self.facing = f2d;
                Some(next)
            } else {
                None
            }
        } else {
            let step = |pos: usize| -> usize {
                match self.facing {
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
                }
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

    /// Returns the size of each cube face.
    fn face_size(&self) -> usize {
        ((self.inner.iter().filter(|&c| *c != Cell::Void).count() / 6) as f32).sqrt() as usize
    }

    /// Returns the cube face a given coordinate is in, as (x, y).
    fn cube_face(&self, idx: usize) -> (usize, usize) {
        let face_size = self.face_size();
        let row = idx / self.width;
        let column = idx % self.width;
        (column / face_size, row / face_size)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Void,
    Empty,
    Wall,
}

#[derive(Copy, Clone, Debug)]
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

impl From<Facing3D> for Facing {
    /// Creates a 2D facing from a 3D one. The 3D facing must not be
    /// In or Out.
    fn from(f: Facing3D) -> Self {
        match f {
            Facing3D::Up => Self::Up,
            Facing3D::Down => Self::Down,
            Facing3D::Left => Self::Left,
            Facing3D::Right => Self::Right,
            _ => unreachable!("unexpected 3d facing"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Facing3D {
    Up,
    Down,
    Left,
    Right,
    In,
    Out,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Facing3D {
    /// Rotate this facing turns times clockwise along the axis.
    fn rotate(&self, axis: Axis, turns: usize) -> Self {
        let one_turn = |f| match (f, axis) {
            (Self::Left, Axis::X) => Self::Left,
            (Self::Right, Axis::X) => Self::Right,
            (Self::Up, Axis::X) => Self::Out,
            (Self::Out, Axis::X) => Self::Down,
            (Self::Down, Axis::X) => Self::In,
            (Self::In, Axis::X) => Self::Up,
            (Self::Up, Axis::Y) => Self::Up,
            (Self::Down, Axis::Y) => Self::Down,
            (Self::Right, Axis::Y) => Self::Out,
            (Self::Out, Axis::Y) => Self::Left,
            (Self::Left, Axis::Y) => Self::In,
            (Self::In, Axis::Y) => Self::Right,
            (Self::In, Axis::Z) => Self::In,
            (Self::Out, Axis::Z) => Self::Out,
            (Self::Up, Axis::Z) => Self::Right,
            (Self::Right, Axis::Z) => Self::Down,
            (Self::Down, Axis::Z) => Self::Left,
            (Self::Left, Axis::Z) => Self::Up,
        };
        let mut rv = *self;
        for _ in 0..turns {
            rv = one_turn(rv);
        }
        rv
    }
}

impl From<Facing> for Facing3D {
    /// Returns the same facing but as 3D.
    fn from(f: Facing) -> Self {
        match f {
            Facing::Up => Self::Up,
            Facing::Down => Self::Down,
            Facing::Left => Self::Left,
            Facing::Right => Self::Right,
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
