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

pub fn solve() -> String {
    let input = include_str!("../inputs/16.txt");
    let (valves, distances) = setup(input);
    format!(
        "{}\n{}",
        part1(&valves, &distances),
        part2(&valves, &distances)
    )
}

fn part1(
    valves: &FxHashMap<String, Valve>,
    distances: &FxHashMap<(String, String), Time>,
) -> Pressure {
    let volcano = Volcano::new(vec![Agent::default()], 30, valves, distances);
    volcano.release_pressure()
}

fn part2(
    valves: &FxHashMap<String, Valve>,
    distances: &FxHashMap<(String, String), Time>,
) -> Pressure {
    let volcano = Volcano::new(
        vec![Agent::default(), Agent::default()],
        26,
        valves,
        distances,
    );
    volcano.release_pressure()
}

fn setup(input: &str) -> (FxHashMap<String, Valve>, FxHashMap<(String, String), Time>) {
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
    (valves, distances)
}

type Time = u8;
type Pressure = u16;

#[derive(Clone, Debug)]
struct Volcano<'a> {
    agents: Vec<Agent<'a>>,
    opened: FxHashSet<&'a str>,
    time_remaining: Time,
    pressure_released: Pressure,
    valves: &'a FxHashMap<String, Valve>,
    distances: &'a FxHashMap<(String, String), Time>,
}

impl<'a> Volcano<'a> {
    fn new(
        agents: Vec<Agent<'a>>,
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
        // If there is no possible way of surpassing the current best
        // found option, just abort right here.
        let potential: u16 = self
            .valves
            .values()
            .filter_map(|v| {
                if self.opened.contains(v.name.as_str()) {
                    None
                } else {
                    let best_eta = self
                        .agents
                        .iter()
                        .map(|agent| {
                            self.distances
                                .get(&(agent.position.to_string(), v.name.to_string()))
                                .unwrap_or(&Time::MAX)
                        })
                        .min()
                        .unwrap();
                    Some(
                        v.flow_rate
                            * ((self.time_remaining as u16).saturating_sub(*best_eta as u16)),
                    )
                }
            })
            .sum();
        if self.pressure_released + potential <= *acc {
            return;
        }

        // Collect the different valves that each agent could open
        // next.
        let mut agent_options: Vec<_> = (0..self.agents.len()).map(|_| Vec::default()).collect();
        for (idx, agent) in self.agents.iter().enumerate() {
            if agent.busy_until < self.time_remaining {
                continue;
            }
            for candidate in self.next_valve_candidates(agent) {
                agent_options[idx].push((idx, candidate));
            }
            agent_options[idx].sort_by_key(|(_, x)| u16::MAX - x.2);
        }

        let idle_agents = agent_options.iter().filter(|o| !o.is_empty()).count();
        *acc = (*acc).max(self.pressure_released);

        if idle_agents == 1 {
            // If there is only one idle agent, branch out for all
            // valves this agent could open next.
            for (idx, (candidate, time_taken, pressure_released)) in
                agent_options.iter().find(|v| !v.is_empty()).unwrap()
            {
                // If there is no possible way of surpassing the
                // current best found option, just abort right here.
                // This is a bit of a mess, because it basically
                // simulates one step ahead before we do an
                // expensive-ish clone of the whole volcano. The
                // functionality is similar, but not the same as the
                // version above.
                let potential: u16 = self
                    .valves
                    .values()
                    .filter_map(|v| {
                        if &v.name == candidate || self.opened.contains(v.name.as_str()) {
                            None
                        } else {
                            let eta_from_candidate = self
                                .distances
                                .get(&(candidate.to_string(), v.name.to_string()))
                                .unwrap_or(&Time::MAX);
                            let best_agent_eta = self
                                .agents
                                .iter()
                                .enumerate()
                                .filter_map(|(i, a)| {
                                    if i != *idx {
                                        Some(
                                            self.distances
                                                .get(&(a.position.to_string(), v.name.to_string()))
                                                .map(|t| *t + self.time_remaining - a.busy_until)
                                                .unwrap_or(Time::MAX),
                                        )
                                    } else {
                                        None
                                    }
                                })
                                .min()
                                .unwrap_or(Time::MAX);
                            let best_eta = eta_from_candidate.min(&best_agent_eta);
                            Some(
                                v.flow_rate
                                    * ((self.time_remaining as u16)
                                        .saturating_sub(*time_taken as u16)
                                        .saturating_sub(*best_eta as u16)),
                            )
                        }
                    })
                    .sum();
                if self.pressure_released + pressure_released + potential <= *acc {
                    continue;
                }

                let mut new_volcano = self.clone();
                let this_agent = &mut new_volcano.agents[*idx];
                this_agent.position = candidate;
                this_agent.busy_until = new_volcano.time_remaining - time_taken;
                new_volcano.time_remaining = new_volcano
                    .agents
                    .iter()
                    .map(|a| a.busy_until)
                    .max()
                    .unwrap_or_default();
                new_volcano.opened.insert(candidate);
                new_volcano.pressure_released += pressure_released;
                new_volcano.pass_time(acc);
            }
        } else {
            // If there are several idle agents, build a catesian
            // product of their options, so that they try all
            // combinations of remaining valves, where they can't open
            // the same valve next.
            for options in agent_options
                .iter()
                .multi_cartesian_product()
                .filter(|opts| opts[0].1 .0 != opts[1].1 .0)
            {
                let mut new_volcano = self.clone();
                for (agent_idx, (next_valve, time_taken, pressure_released)) in options {
                    let this_agent = &mut new_volcano.agents[*agent_idx];
                    this_agent.position = next_valve;
                    this_agent.busy_until = new_volcano.time_remaining - time_taken;
                    new_volcano.opened.insert(next_valve);
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
            .filter(|v| !self.opened.contains(v.name.as_str()))
            .filter_map(|v| {
                let time_required: Time = *self
                    .distances
                    .get(&(agent.position.to_string(), v.name.to_string()))
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
struct Agent<'a> {
    position: &'a str,
    busy_until: Time,
}

impl<'a> Default for Agent<'a> {
    fn default() -> Self {
        Self {
            position: "AA",
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
        let (v, d) = setup(INPUT);
        assert_eq!(part1(&v, &d), 1651);
    }

    #[test]
    fn part2_example() {
        let (v, d) = setup(INPUT);
        assert_eq!(part2(&v, &d), 1707);
    }
}
