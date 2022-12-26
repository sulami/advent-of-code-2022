use fxhash::FxHashSet;

pub fn solve() -> String {
    let input = include_str!("../inputs/06.txt");
    format!(
        "{}\n{}",
        find_start(input, 4).expect("unable to find start"),
        find_start(input, 14).expect("unable to find start")
    )
}

fn find_start(message: &str, size: usize) -> Option<usize> {
    (0..message.len() - size)
        .find(|&i| {
            message
                .chars()
                .skip(i)
                .take(size)
                .collect::<FxHashSet<_>>()
                .len()
                == size
        })
        .map(|i| i + size)
}
