use rayon::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let start = std::time::Instant::now();
    let mut outputs = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
        day20::solve,
        day21::solve,
        day22::solve,
        day23::solve,
        day24::solve,
        day25::solve,
    ]
    .par_iter()
    .enumerate()
    .map(|(day, fun)| {
        let day_start = std::time::Instant::now();
        let mut output = format!("Day {}:\n{}", day + 1, fun());
        if !std::env::var("TIME").unwrap_or_default().is_empty() {
            output = format!("{output}\ntook {:?}", day_start.elapsed());
        }
        output
    })
    .collect::<Vec<String>>();
    if !std::env::var("TIME").unwrap_or_default().is_empty() {
        outputs.push(format!("total: {:?}", start.elapsed()));
    }
    outputs.iter().for_each(|o| println!("{o}"));
}
