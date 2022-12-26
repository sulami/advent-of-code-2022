pub fn solve() -> String {
    let input = include_str!("../inputs/01.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    *elf_calories(input).last().unwrap()
}

fn part2(input: &str) -> u32 {
    elf_calories(input).iter().rev().take(3).sum()
}

fn elf_calories(input: &str) -> Vec<u32> {
    let count_calories = |elf: &str| -> u32 {
        elf.split_whitespace()
            .map(|n| n.parse::<u32>().expect("failed to parse calories"))
            .sum()
    };
    let mut elves: Vec<u32> = input.split("\n\n").map(count_calories).collect();
    elves.sort();
    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 45000);
    }
}
