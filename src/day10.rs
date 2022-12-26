use std::str::FromStr;

pub fn solve() -> String {
    let input = include_str!("../inputs/10.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> i16 {
    parse_and_run(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| (i as i16 + 1) * x)
        .sum()
}

fn part2(input: &str) -> String {
    parse_and_run(input)
        .enumerate()
        .map(|(beam, x)| {
            format!(
                "{}{}",
                if (x % 40).abs_diff(beam as i16 % 40) <= 1 {
                    "#"
                } else {
                    "."
                },
                if beam % 40 == 39 { "\n" } else { "" }
            )
        })
        .collect()
}

/// Parses and runs instructions in input, returning an iterator of X
/// register values for each CPU cycle.
fn parse_and_run(input: &str) -> impl Iterator<Item = i16> + '_ {
    let mut cpu = Cpu::default();
    input
        .lines()
        .map(|l| l.parse::<Instruction>().expect("invalid instruction"))
        .flat_map(move |i| cpu.run(&i))
}

enum Instruction {
    Noop,
    AddX(i16),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }
        if let Some(("addx", n)) = s.split_once(' ') {
            return Ok(Self::AddX(n.parse().map_err(|_| "invalid addx value")?));
        }
        Err("invalid instruction")
    }
}

#[derive(Copy, Clone)]
struct Cpu {
    x: i16,
}

impl Default for Cpu {
    fn default() -> Self {
        Self { x: 1 }
    }
}

impl Cpu {
    /// Runs an instruction and returns the values of the x register
    /// during all cycles consumed.
    fn run(&mut self, instruction: &Instruction) -> Vec<i16> {
        match instruction {
            Instruction::Noop => vec![self.x],
            Instruction::AddX(n) => {
                let rv = vec![self.x, self.x];
                self.x += n;
                rv
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13140);
    }

    #[test]
    fn part2_example() {
        let expected = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(part2(INPUT), expected);
    }
}
