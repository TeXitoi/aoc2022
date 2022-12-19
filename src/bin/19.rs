use regex::Regex;
use std::io::{self, BufRead};

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(
            "^Blueprint (\\d+): \
             Each ore robot costs (\\d+) ore. \
             Each clay robot costs (\\d+) ore. \
             Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
             Each geode robot costs (\\d+) ore and (\\d+) obsidian.$"
        ).unwrap();
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    remaining: i32,
    nb_geode: i32,
    nb_ore_robots: i32,
    nb_ore: i32,
    nb_clay_robots: i32,
    nb_clay: i32,
    nb_obsidian_robots: i32,
    nb_obsidian: i32,
}
impl State {
    fn step(&mut self) {
        self.remaining -= 1;
        self.nb_ore += self.nb_ore_robots;
        self.nb_clay += self.nb_clay_robots;
        self.nb_obsidian += self.nb_obsidian_robots;
    }
    fn is_dominated_by(&self, other: &Self) -> bool {
        self.remaining <= other.remaining
            && self.nb_geode <= other.nb_geode
            && self.nb_ore + self.remaining * self.nb_ore_robots
                <= other.nb_ore + other.remaining * other.nb_ore_robots
            && self.nb_clay + self.remaining * self.nb_clay_robots
                <= other.nb_clay + other.remaining * other.nb_clay_robots
            && self.nb_obsidian + self.remaining * self.nb_obsidian_robots
                <= other.nb_obsidian + other.remaining * other.nb_obsidian_robots
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    nb_ore_for_ore: i32,
    nb_ore_for_clay: i32,
    nb_ore_clay_for_obsidian: (i32, i32),
    nb_ore_obsidian_for_geode: (i32, i32),
}
impl<'a> TryFrom<&'a str> for Blueprint {
    type Error = anyhow::Error;
    fn try_from(s: &'a str) -> anyhow::Result<Self> {
        let Some(c) = RE.captures(s) else { anyhow::bail!("bad blueprint {:?}", s) };
        Ok(Self {
            id: c[1].parse()?,
            nb_ore_for_ore: c[2].parse()?,
            nb_ore_for_clay: c[3].parse()?,
            nb_ore_clay_for_obsidian: (c[4].parse()?, c[5].parse()?),
            nb_ore_obsidian_for_geode: (c[6].parse()?, c[7].parse()?),
        })
    }
}
impl Blueprint {
    fn quality_level(&self) -> i32 {
        self.id * self.nb_geodes(24)
    }
    fn next(&self, state: State) -> impl Iterator<Item = State> {
        self.make_geode_robot(state)
            .into_iter()
            .chain(self.make_obsidian_robot(state))
            .chain(self.make_clay_robot(state))
            .chain(self.make_ore_robot(state))
    }
    fn nb_geodes(&self, remaining: i32) -> i32 {
        let state = State {
            remaining,
            nb_ore_robots: 1,
            ..Default::default()
        };
        let mut q = std::collections::BinaryHeap::from(vec![state]);
        let mut solutions = vec![state];
        while let Some(old_state) = q.pop() {
            for state in self.next(old_state) {
                if solutions.iter().any(|s| state.is_dominated_by(s)) {
                    continue;
                }
                q.push(state);
                solutions.retain(|s| !s.is_dominated_by(&state));
                solutions.push(state);
            }
        }
        solutions.iter().map(|s| s.nb_geode).max().unwrap_or(0)
    }
    fn make_ore_robot(&self, mut state: State) -> Option<State> {
        if state.nb_ore_robots
            >= self
                .nb_ore_for_ore
                .max(self.nb_ore_for_clay)
                .max(self.nb_ore_clay_for_obsidian.0)
                .max(self.nb_ore_obsidian_for_geode.0)
        {
            return None;
        }
        loop {
            if state.remaining <= 2 {
                return None;
            }
            if state.nb_ore >= self.nb_ore_for_ore {
                state.nb_ore -= self.nb_ore_for_ore;
                state.step();
                state.nb_ore_robots += 1;
                return Some(state);
            }
            state.step();
        }
    }
    fn make_clay_robot(&self, mut state: State) -> Option<State> {
        if state.nb_clay_robots >= self.nb_ore_clay_for_obsidian.1 {
            return None;
        }
        loop {
            if state.remaining <= 2 {
                return None;
            }
            if state.nb_ore >= self.nb_ore_for_clay {
                state.nb_ore -= self.nb_ore_for_clay;
                state.step();
                state.nb_clay_robots += 1;
                return Some(state);
            }
            state.step();
        }
    }
    fn make_obsidian_robot(&self, mut state: State) -> Option<State> {
        if state.nb_obsidian_robots >= self.nb_ore_obsidian_for_geode.1 {
            return None;
        }
        loop {
            if state.remaining <= 2 {
                return None;
            }
            if state.nb_ore >= self.nb_ore_clay_for_obsidian.0
                && state.nb_clay >= self.nb_ore_clay_for_obsidian.1
            {
                state.nb_ore -= self.nb_ore_clay_for_obsidian.0;
                state.nb_clay -= self.nb_ore_clay_for_obsidian.1;
                state.step();
                state.nb_obsidian_robots += 1;
                return Some(state);
            }
            state.step();
        }
    }
    fn make_geode_robot(&self, mut state: State) -> Option<State> {
        loop {
            if state.remaining <= 1 {
                return None;
            }
            if state.nb_ore >= self.nb_ore_obsidian_for_geode.0
                && state.nb_obsidian >= self.nb_ore_obsidian_for_geode.1
            {
                state.nb_ore -= self.nb_ore_obsidian_for_geode.0;
                state.nb_obsidian -= self.nb_ore_obsidian_for_geode.1;
                state.step();
                state.nb_geode += state.remaining;
                return Some(state);
            }
            state.step();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let blueprints = io::BufReader::new(std::fs::File::open("data/input19.txt")?)
        .lines()
        .map(|l| Ok(Blueprint::try_from(&*l?)?))
        .collect::<anyhow::Result<Vec<Blueprint>>>()?;
    let mut quality_levels = 0;
    for blueprint in &blueprints {
        quality_levels += blueprint.quality_level();
    }
    println!("Part1: {}", quality_levels);

    let mut product = 1;
    for blueprint in blueprints.iter().take(3) {
        product *= blueprint.nb_geodes(32);
    }
    println!("Part2: {}", product);

    Ok(())
}
