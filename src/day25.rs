use itertools::iterate;

pub fn solve() -> String {
    let input = include_str!("../inputs/25.txt");
    format!("{}", part1(input))
}

fn part1(input: &str) -> String {
    decimal_to_snafu(input.lines().map(snafu_to_decimal).sum())
}

/// Takes a number in SNAFU format and returns a decimal number.
fn snafu_to_decimal(i: &str) -> i64 {
    let places = iterate(1, |n| n * 5);
    let digit_value = |c| match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("invalid SNAFU digit"),
    };
    i.chars()
        .rev()
        .zip(places)
        .map(|(c, value)| value * digit_value(c))
        .sum()
}

/// Takes a decimal number and returns a number in SNAFU format. Only
/// handles integers >= 0.
fn decimal_to_snafu(i: i64) -> String {
    if i == 0 {
        return "0".to_string();
    }
    // Find the top digit.
    let top_place = iterate(1, |n| n * 5)
        .take_while(|n| i * 2 >= *n)
        .last()
        .unwrap();
    let digit = |n| match n {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("oh no"),
    };
    let first_digit = (1..=2).min_by_key(|n| i.abs_diff(n * top_place)).unwrap();
    let mut rv = String::from(digit(first_digit));
    if top_place == 1 {
        return rv;
    }
    let mut current = top_place * first_digit;
    for place in iterate(top_place / 5, |n| n / 5) {
        let next_digit = (-2..=2)
            .min_by_key(|n| (current + n * place).abs_diff(i))
            .unwrap();
        current += next_digit * place;
        rv.push(digit(next_digit));
        if place <= 1 {
            break;
        }
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), "2=-1=0");
    }
}
