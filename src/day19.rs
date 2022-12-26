use nom::{bytes::complete::tag, character::complete::u8, combinator::all_consuming, IResult};

pub fn solve() -> String {
    let input = include_str!("../inputs/19.txt");
    format!("{}\n{}", part1(input), part2(input))
}

fn part1(input: &str) -> u32 {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .map(|l| {
            all_consuming(parse_blueprint)(l)
                .expect("failed to parse blueprint")
                .1
        })
        .collect();

    blueprints.iter().map(Blueprint::quality_level).sum()
}

fn part2(input: &str) -> u32 {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .take(3)
        .map(|l| {
            all_consuming(parse_blueprint)(l)
                .expect("failed to parse blueprint")
                .1
        })
        .collect();

    blueprints
        .iter()
        .map(|bp| bp.max_geodes(32) as u32)
        .product()
}

type Ore = u8;
type Clay = u8;
type Obsidian = u8;
type Geodes = u8;

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot_cost: Ore,
    clay_robot_cost: Ore,
    obsidian_robot_cost: (Ore, Clay),
    geode_robot_cost: (Ore, Obsidian),
}

#[derive(Copy, Clone, Debug)]
struct EvaluationState {
    time: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

impl Blueprint {
    /// Calculates the quality level for part 1.
    fn quality_level(&self) -> u32 {
        self.id as u32 * self.max_geodes(24) as u32
    }

    /// Returns the maximum number of geodes that can be cracked in a
    /// given time.
    fn max_geodes(&self, time_limit: u8) -> u8 {
        let mut most_geodes = 0;
        let mut earliest_geode = 25;

        let initial_state = EvaluationState {
            time: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        };
        self.find_max_geodes(
            initial_state,
            time_limit,
            &mut most_geodes,
            &mut earliest_geode,
        );
        most_geodes
    }

    /// Returns the maximum per-minute ore production we'd ever need.
    fn max_ore_need(&self) -> Ore {
        *[
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost.0,
            self.geode_robot_cost.0,
        ]
        .iter()
        .max()
        .unwrap()
    }

    /// Recursively evaluates all reasonable build orders to find the
    /// maximum amount of geodes that can be produced.
    fn find_max_geodes(
        &self,
        state: EvaluationState,
        time_limit: u8,
        most_geodes: &mut Geodes,
        earliest_geode: &mut u8,
    ) {
        let time_left = (time_limit - state.time) as u32;
        if (state.time > *earliest_geode && state.geode_robots == 0)
            || state.geodes as u32
                + time_left * state.geode_robots as u32
                + (1..=time_left).sum::<u32>()
                < *most_geodes as u32
        {
            return;
        } else if state.geode_robots > 0 {
            *earliest_geode = (*earliest_geode).min(state.time);
        }

        // Returns the time it takes to reach a resource goal at
        // current production levels, plus 1 to actually build a
        // robot.
        let time_to_resource = |current: u8, target: u8, production: u8| -> u8 {
            if production == 0 {
                u8::MAX
            } else if target <= current {
                1
            } else {
                div_ceil(target.saturating_sub(current), production) + 1
            }
        };

        // Returns the time it takes to build a robot given
        // time_to_resource-style tuples.
        let time_to_robot = |resources: &[(u8, u8, u8)]| -> u8 {
            resources
                .iter()
                .map(|(c, t, p)| time_to_resource(*c, *t, *p))
                .max()
                .unwrap()
        };

        let geode_costs = [
            (state.ore, self.geode_robot_cost.0, state.ore_robots),
            (
                state.obsidian,
                self.geode_robot_cost.1,
                state.obsidian_robots,
            ),
        ];
        let obsidian_costs = [
            (state.ore, self.obsidian_robot_cost.0, state.ore_robots),
            (state.clay, self.obsidian_robot_cost.1, state.clay_robots),
        ];
        let clay_costs = [(state.ore, self.clay_robot_cost, state.ore_robots)];
        let ore_costs = [(state.ore, self.ore_robot_cost, state.ore_robots)];

        // Times to get the next robots if we just wait.
        let time_to_geode = time_to_robot(&geode_costs);
        let time_to_obsidian = time_to_robot(&obsidian_costs);
        let time_to_clay = time_to_robot(&clay_costs);
        let time_to_ore = time_to_robot(&ore_costs);

        if state.time.saturating_add(time_to_geode) <= time_limit {
            let mut fork = state;
            fork.time += time_to_geode;

            fork.ore += fork.ore_robots * time_to_geode;
            fork.clay += fork.clay_robots * time_to_geode;
            fork.obsidian += fork.obsidian_robots * time_to_geode;
            fork.geodes += fork.geode_robots * time_to_geode;

            fork.geode_robots += 1;
            fork.ore -= self.geode_robot_cost.0;
            fork.obsidian -= self.geode_robot_cost.1;

            *most_geodes = (*most_geodes).max(fork.geodes);

            if fork.time < time_limit {
                self.find_max_geodes(fork, time_limit, most_geodes, earliest_geode);
            }
        }
        if state.obsidian_robots < self.geode_robot_cost.1
            && state.time.saturating_add(time_to_obsidian) <= time_limit
        {
            let mut fork = state;
            fork.time += time_to_obsidian;

            fork.ore += fork.ore_robots * time_to_obsidian;
            fork.clay += fork.clay_robots * time_to_obsidian;
            fork.obsidian += fork.obsidian_robots * time_to_obsidian;
            fork.geodes += fork.geode_robots * time_to_obsidian;

            fork.obsidian_robots += 1;
            fork.ore -= self.obsidian_robot_cost.0;
            fork.clay -= self.obsidian_robot_cost.1;

            *most_geodes = (*most_geodes).max(fork.geodes);

            if fork.time < time_limit {
                self.find_max_geodes(fork, time_limit, most_geodes, earliest_geode);
            }
        }
        if state.clay_robots < self.obsidian_robot_cost.1
            && state.time.saturating_add(time_to_clay) <= time_limit
        {
            let mut fork = state;
            fork.time += time_to_clay;

            fork.ore += fork.ore_robots * time_to_clay;
            fork.clay += fork.clay_robots * time_to_clay;
            fork.obsidian += fork.obsidian_robots * time_to_clay;
            fork.geodes += fork.geode_robots * time_to_clay;

            fork.clay_robots += 1;
            fork.ore -= self.clay_robot_cost;

            *most_geodes = (*most_geodes).max(fork.geodes);

            if fork.time < time_limit {
                self.find_max_geodes(fork, time_limit, most_geodes, earliest_geode);
            }
        }
        if state.ore_robots < self.max_ore_need()
            && state.time.saturating_add(time_to_ore) <= time_limit
        {
            let mut fork = state;
            fork.time += time_to_ore;

            fork.ore += fork.ore_robots * time_to_ore;
            fork.clay += fork.clay_robots * time_to_ore;
            fork.obsidian += fork.obsidian_robots * time_to_ore;
            fork.geodes += fork.geode_robots * time_to_ore;

            fork.ore_robots += 1;
            fork.ore -= self.ore_robot_cost;

            *most_geodes = (*most_geodes).max(fork.geodes);

            if fork.time < time_limit {
                self.find_max_geodes(fork, time_limit, most_geodes, earliest_geode);
            }
        }
    }
}

/// Divides and rounds up. Actual div_ceil is still unstable, and I
/// don't want to use a nightly version, so here's a specialised port.
fn div_ceil(lhs: u8, rhs: u8) -> u8 {
    assert!(rhs != 0);
    lhs / rhs + (lhs % rhs).min(1)
}

fn parse_blueprint(i: &str) -> IResult<&str, Blueprint> {
    let (i, _) = tag("Blueprint ")(i)?;
    let (i, id) = u8(i)?;
    let (i, _) = tag(": Each ore robot costs ")(i)?;
    let (i, ore_robot_cost) = u8(i)?;
    let (i, _) = tag(" ore. Each clay robot costs ")(i)?;
    let (i, clay_robot_cost) = u8(i)?;
    let (i, _) = tag(" ore. Each obsidian robot costs ")(i)?;
    let (i, obsidian_robot_ore_cost) = u8(i)?;
    let (i, _) = tag(" ore and ")(i)?;
    let (i, obsidian_robot_clay_cost) = u8(i)?;
    let (i, _) = tag(" clay. Each geode robot costs ")(i)?;
    let (i, geode_robot_ore_cost) = u8(i)?;
    let (i, _) = tag(" ore and ")(i)?;
    let (i, geode_robot_obsidian_cost) = u8(i)?;
    let (i, _) = tag(" obsidian.")(i)?;
    Ok((
        i,
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost: (obsidian_robot_ore_cost, obsidian_robot_clay_cost),
            geode_robot_cost: (geode_robot_ore_cost, geode_robot_obsidian_cost),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 33);
    }

    // Slow
    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(INPUT), 56 * 62);
    // }
}
