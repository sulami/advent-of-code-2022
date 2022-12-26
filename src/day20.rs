use itertools::Itertools;

pub fn solve() -> String {
    let input = include_str!("../inputs/20.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> i64 {
    let mut nums: Vec<(usize, i64)> = input
        .lines()
        .map(|l| l.parse().expect("failed to parse file"))
        .enumerate()
        .collect();

    mix(&mut nums);

    let zero_position = nums.iter().position(|(_, n)| *n == 0).unwrap();
    let a = nums[(zero_position + 1000) % nums.len()].1;
    let b = nums[(zero_position + 2000) % nums.len()].1;
    let c = nums[(zero_position + 3000) % nums.len()].1;
    a + b + c
}

fn part2(input: &str) -> i64 {
    let decryption_key = 811589153;
    let mut nums: Vec<(usize, i64)> = input
        .lines()
        .map(|l| l.parse().expect("failed to parse file"))
        .map(|n: i64| n * decryption_key)
        .enumerate()
        .collect();

    for _ in 0..10 {
        mix(&mut nums);
    }

    let zero_position = nums.iter().position(|(_, n)| *n == 0).unwrap();
    let a = nums[(zero_position + 1000) % nums.len()].1;
    let b = nums[(zero_position + 2000) % nums.len()].1;
    let c = nums[(zero_position + 3000) % nums.len()].1;
    a + b + c
}

/// Mixes nums once, according to the instructions.
fn mix(nums: &mut Vec<(usize, i64)>) {
    for idx in 0..nums.len() {
        let (current_index, (order, num)) = nums.iter().find_position(|(i, _)| *i == idx).unwrap();
        // Copy values to avoid ownership issues.
        let num = *num;
        let order = *order;

        // Just skip numbers that don't need to move.
        if num == 0 {
            continue;
        }

        nums.remove(current_index);

        let raw_new_index = current_index as i64 + num;
        let new_index: i64 = if num > 0 {
            raw_new_index % nums.len() as i64
        } else if raw_new_index < 0 {
            nums.len() as i64 - (raw_new_index.abs() % nums.len() as i64)
        } else {
            raw_new_index
        };
        nums.insert(new_index as usize, (order, num));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1
2
-3
3
-2
0
4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1623178306);
    }
}
