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
    let volcano = Volcano::new(vec![Agent::default()], 30, &valves, &distances);
    volcano.release_pressure()
}

fn part2(input: &str) -> Pressure {
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
    let volcano = Volcano::new(
        vec![Agent::default(), Agent::default()],
        26,
        &valves,
        &distances,
    );
    volcano.release_pressure()
}

type Time = u8;
type Pressure = u16;

#[derive(Clone, Debug)]
struct Volcano<'a> {
    agents: Vec<Agent>,
    opened: FxHashSet<String>,
    time_remaining: Time,
    pressure_released: Pressure,
    valves: &'a FxHashMap<String, Valve>,
    distances: &'a FxHashMap<(String, String), Time>,
}

impl<'a> Volcano<'a> {
    fn new(
        agents: Vec<Agent>,
        time_remaining: Time,
        valves: &'a FxHashMap<String, Valve>,
        distances: &'a FxHashMap<(String, String), Time>,
    ) -> Self {
        Self {
            agents,
            opened: FxHashSet::default(),
            time_remaining,
            pressure_released: 0,
            valves,
            distances,
        }
    }

    /// Returns the maximum amount of pressure that can be released in
    /// this volcano.
    fn release_pressure(self) -> Pressure {
        let mut rv = 0;
        self.pass_time(&mut rv);
        rv
    }

    /// Called the next time at least one agent is idle, and thus
    /// needs to get moving.
    fn pass_time(self, acc: &mut Pressure) {
        let mut agent_options: Vec<_> = (0..self.agents.len())
            .map(|_| FxHashSet::default())
            .collect();
        for (idx, agent) in self.agents.iter().enumerate() {
            if agent.busy_until < self.time_remaining {
                continue;
            }
            for candidate in self.next_valve_candidates(agent) {
                agent_options[idx].insert((idx, candidate));
            }
        }
        let idle_agents = agent_options.iter().filter(|o| !o.is_empty()).count();
        *acc = (*acc).max(self.pressure_released);
        if idle_agents == 1 {
            for (idx, (candidate, time_taken, pressure_released)) in
                agent_options.iter().find(|v| !v.is_empty()).unwrap()
            {
                let mut new_volcano = self.clone();
                let this_agent = &mut new_volcano.agents[*idx];
                this_agent.position = candidate.clone();
                this_agent.busy_until = new_volcano.time_remaining - time_taken;
                new_volcano.time_remaining = new_volcano
                    .agents
                    .iter()
                    .map(|a| a.busy_until)
                    .max()
                    .unwrap_or_default();
                new_volcano.opened.insert(candidate.clone());
                new_volcano.pressure_released += pressure_released;
                new_volcano.pass_time(acc);
            }
        } else {
            for options in agent_options
                .iter()
                .multi_cartesian_product()
                .filter(|opts| opts[0].1 .0 != opts[1].1 .0)
            {
                let mut new_volcano = self.clone();
                for (agent_idx, (next_valve, time_taken, pressure_released)) in options {
                    let this_agent = &mut new_volcano.agents[*agent_idx];
                    this_agent.position = next_valve.clone();
                    this_agent.busy_until = new_volcano.time_remaining - time_taken;
                    new_volcano.opened.insert(next_valve.clone());
                    new_volcano.pressure_released += pressure_released;
                }
                new_volcano.time_remaining = new_volcano
                    .agents
                    .iter()
                    .map(|a| a.busy_until)
                    .max()
                    .unwrap_or_default();
                new_volcano.pass_time(acc);
            }
        }
    }

    /// Returns a Vec of valves that could be opened next, along with
    /// the time that would take, and the pressure that would be
    /// released in the time remaining.
    fn next_valve_candidates(&self, agent: &Agent) -> Vec<(String, Time, Pressure)> {
        self.valves
            .values()
            .filter(|v| v.flow_rate > 0)
            .filter(|v| !self.opened.contains(&v.name))
            .filter_map(|v| {
                let time_required: Time = *self
                    .distances
                    .get(&(agent.position.clone(), v.name.clone()))
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Agent {
    position: String,
    busy_until: Time,
}

impl Default for Agent {
    fn default() -> Self {
        Self {
            position: "AA".to_string(),
            busy_until: 30,
        }
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1707);
    }
}
