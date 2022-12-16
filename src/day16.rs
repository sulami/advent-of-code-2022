use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u16},
    combinator::{all_consuming, map},
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

pub fn solve() {
    let input = include_str!("../inputs/16.txt");
    println!("day 16-1: {}", part1(input));
    println!("day 16-2: {}", part2(input));
}

fn part1(input: &str) -> Pressure {
    let valves: FxHashMap<_, _> = input
        .lines()
        .map(|l| {
            all_consuming(parse_valve)(l)
                .expect("failed to parse valve")
                .1
        })
        .map(|v| (v.name.clone(), v))
        .collect();
    // These are the valves that can actually be opened, as well as
    // the starting position. We use this to pre-calculate distances
    // between useful graph nodes. The useless valves inbetween are
    // just used to modify edge weights.
    let mut useful_valves: Vec<Valve> = valves
        .values()
        .cloned()
        .filter(|v| v.flow_rate > 0)
        .collect();
    useful_valves.push(valves.get("AA").unwrap().clone());
    let distances: FxHashMap<_, _> = useful_valves
        .iter()
        .permutations(2)
        .par_bridge()
        .map(|vs| {
            (
                (vs[0].name.clone(), vs[1].name.clone()),
                vs[0].eta(&vs[1].name, &valves),
            )
        })
        .collect();
    let mut me = Me::new(&valves, &distances);
    me.release_pressure()
}

fn part2(_input: &str) -> usize {
    0
}

type Time = u8;
type Pressure = u16;

#[derive(Clone)]
struct Me<'a> {
    position: String,
    opened: FxHashSet<String>,
    time_remaining: Time,
    pressure_released: Pressure,
    valves: &'a FxHashMap<String, Valve>,
    distances: &'a FxHashMap<(String, String), Time>,
}

impl<'a> Me<'a> {
    /// Constructs a new me!
    fn new(
        valves: &'a FxHashMap<String, Valve>,
        distances: &'a FxHashMap<(String, String), Time>,
    ) -> Self {
        Self {
            position: "AA".to_string(),
            opened: FxHashSet::default(),
            time_remaining: 30,
            pressure_released: 0,
            valves,
            distances,
        }
    }

    /// Returns the maximum amount of pressure I can release.
    fn release_pressure(&mut self) -> Pressure {
        let mut rv = 0;
        self.open_valves(&mut rv);
        rv
    }

    /// Recursively try all different valve combinations that can be
    /// tried in the time remaining. Update acc with a new released
    /// pressure if I find an order that releases more pressure than
    /// acc currently holds.
    fn open_valves(&mut self, acc: &mut Pressure) {
        let candidates = self.next_valve_candidates();
        if candidates.is_empty() {
            *acc = (*acc).max(self.pressure_released);
        }
        for (candidate, time_taken, pressure_released) in candidates {
            let mut opened = self.opened.clone();
            opened.insert(candidate.clone());
            let mut new_me = Self {
                position: candidate,
                time_remaining: self.time_remaining - time_taken,
                pressure_released: self.pressure_released + pressure_released,
                valves: self.valves,
                opened,
                distances: self.distances,
            };
            new_me.open_valves(acc);
        }
    }

    /// Returns a Vec of valves that could be opened next, along with
    /// the time that would take, and the pressure that would be
    /// released in the time remaining.
    fn next_valve_candidates(&self) -> Vec<(String, Time, Pressure)> {
        self.valves
            .values()
            .filter(|v| v.flow_rate > 0)
            .filter(|v| !self.opened.contains(&v.name))
            .filter_map(|v| {
                let time_required: Time = *self
                    .distances
                    .get(&(self.position.clone(), v.name.clone()))
                    .unwrap();
                if time_required <= self.time_remaining {
                    Some((
                        v.name.clone(),
                        time_required,
                        (self.time_remaining - time_required) as Pressure * v.flow_rate,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    flow_rate: Pressure,
    leads_to: Vec<String>,
}

impl Valve {
    /// Returns the number of minutes it takes to reach and open
    /// another valve when standing in front of this one.
    fn eta(&self, to: &str, valves: &FxHashMap<String, Valve>) -> Time {
        let mut options = VecDeque::default();
        self.leads_to.iter().for_each(|v| options.push_back((v, 1)));
        while let Some((name, distance)) = options.pop_front() {
            if name == to {
                return distance + 1;
            }
            valves
                .get(name)
                .unwrap()
                .leads_to
                .iter()
                .for_each(|v| options.push_back((v, distance + 1)));
        }
        panic!("Failed to find a path to target valve");
    }
}

fn parse_valve(i: &str) -> IResult<&str, Valve> {
    let (i, _) = tag("Valve ")(i)?;
    let (i, name) = map(alpha1, str::to_string)(i)?;
    let (i, _) = tag(" has flow rate=")(i)?;
    let (i, flow_rate) = u16(i)?;
    let (i, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(i)?;
    let (i, leads_to) = separated_list1(tag(", "), map(alpha1, str::to_string))(i)?;
    Ok((
        i,
        Valve {
            name,
            flow_rate,
            leads_to,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 1651);
    }
}
