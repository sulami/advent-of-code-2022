use fxhash::FxHashSet;
use nom::{bytes::complete::tag, character::complete::i32, combinator::all_consuming, IResult};
use rayon::prelude::*;

pub fn solve() -> String {
    let input = include_str!("../inputs/15.txt");
    format!("{}\n{}", part1(input, 2_000_000), part2(input, 4_000_000))
}

fn part1(input: &str, row: i32) -> usize {
    let (sensors, _beacons) = parse_sensors_and_beacons(input);
    let mut ranges: Vec<_> = sensors.iter().map(|s| s.row_coverage(row)).collect();
    ranges.sort_by_key(|c| c.0);
    let mut combined_ranges = vec![];
    ranges.iter().for_each(|(start, end)| {
        if let Some((_, last_end)) = combined_ranges.last() {
            if start <= last_end {
                combined_ranges.last_mut().unwrap().1 = *end.max(last_end);
            } else {
                combined_ranges.push((*start, *end));
            }
        } else {
            combined_ranges.push((*start, *end));
        }
    });
    combined_ranges
        .iter()
        .map(|(start, end)| end.abs_diff(*start) as usize)
        .sum()
}

fn part2(input: &str, limits: i32) -> i64 {
    let (sensors, beacons) = parse_sensors_and_beacons(input);
    let (x, y) = sensors
        .par_iter()
        .flat_map(|s| s.just_out_of_reach(limits, &sensors))
        .find_any(|b| !beacons.contains(b))
        .expect("failed to find beacon");
    x as i64 * 4_000_000 + y as i64
}

fn parse_sensors_and_beacons(input: &str) -> (Vec<Sensor>, FxHashSet<(i32, i32)>) {
    let sensors: Vec<_> = input
        .lines()
        .map(|l| {
            all_consuming(parse_sensor)(l)
                .expect("failed to parse sensor")
                .1
        })
        .collect();
    let beacons: FxHashSet<_> = sensors.iter().map(|s| s.closest_beacon).collect();
    (sensors, beacons)
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    closest_beacon: (i32, i32),
    detection_range: i32,
}

impl Sensor {
    /// Returns true if the field at coords is covered by this sensor.
    fn is_covered(&self, coords: &(i32, i32)) -> bool {
        manhattan_distance(&(self.x, self.y), coords) <= self.detection_range
    }

    /// Returns a pair of lower and upper bound of all x-coordinates
    /// covered by this sensor which match a given y-coordinate.
    fn row_coverage(&self, y: i32) -> (i32, i32) {
        let range = self.detection_range - self.y.abs_diff(y) as i32;
        let a = self.x - range;
        let b = self.x + range;
        (a.min(b), a.max(b))
    }

    /// Returns the fields that are surrounding the coverage of this
    /// sensor, i.e. manhattan distance = detection range + 1. For
    /// performance reasons this only includes fields where 0 <= field
    /// <= limits.
    fn just_out_of_reach(&self, limits: i32, sensors: &[Sensor]) -> Vec<(i32, i32)> {
        let detection_range = self.detection_range + 1;
        for y_offset in -detection_range..=detection_range {
            let y = self.y + y_offset;
            if y < 0 || y > limits {
                continue;
            }
            let x_offset = detection_range - y_offset.abs();
            let x = self.x - x_offset;
            if x >= 0 && x <= limits && !sensors.iter().any(|s| s.is_covered(&(x, y))) {
                return vec![(x, y)];
            }
            let x = self.x + x_offset;
            if x >= 0 && x <= limits && !sensors.iter().any(|s| s.is_covered(&(x, y))) {
                return vec![(x, y)];
            }
        }
        vec![]
    }
}

/// Returns the manhattan distance between two points.
fn manhattan_distance(left: &(i32, i32), right: &(i32, i32)) -> i32 {
    (left.0.abs_diff(right.0) + left.1.abs_diff(right.1)) as i32
}

fn parse_sensor(i: &str) -> IResult<&str, Sensor> {
    let (i, _) = tag("Sensor at x=")(i)?;
    let (i, x) = i32(i)?;
    let (i, _) = tag(", y=")(i)?;
    let (i, y) = i32(i)?;
    let (i, _) = tag(": closest beacon is at x=")(i)?;
    let (i, bx) = i32(i)?;
    let (i, _) = tag(", y=")(i)?;
    let (i, by) = i32(i)?;
    let detection_range = (x.abs_diff(bx) + y.abs_diff(by)) as i32;
    Ok((
        i,
        Sensor {
            x,
            y,
            closest_beacon: (bx, by),
            detection_range,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT, 10), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT, 20), 56000011);
    }
}
