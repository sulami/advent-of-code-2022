pub fn solve() -> String {
    let input = include_str!("../inputs/02.txt");
    format!("{}\n{}", part1(input), part2(input))
}

#[allow(clippy::identity_op)]
fn part1(input: &str) -> i32 {
    let strategy = |plays: &str| -> i32 {
        match plays {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("invalid play"),
        }
    };
    calculate(input, strategy)
}

#[allow(clippy::identity_op)]
fn part2(input: &str) -> i32 {
    let strategy = |plays: &str| -> i32 {
        match plays {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("invalid play"),
        }
    };
    calculate(input, strategy)
}

fn calculate(input: &str, strategy: fn(&str) -> i32) -> i32 {
    input.lines().map(strategy).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 12);
    }
}
