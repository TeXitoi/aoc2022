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
    remaining: u32,
    nb_geode: u32,
    nb_ore_robots: u32,
    nb_ore: u32,
    nb_clay_robots: u32,
    nb_clay: u32,
    nb_obsidian_robots: u32,
    nb_obsidian: u32,
}
impl State {
    fn step(&mut self) {
        self.remaining -= 1;
        self.nb_ore += self.nb_ore_robots;
        self.nb_clay += self.nb_clay_robots;
        self.nb_obsidian += self.nb_obsidian_robots;
    }
    fn is_dominated_by(&self, other: &Self) -> bool {
        self.nb_geode <= other.nb_geode
            && self.nb_ore <= other.nb_ore
            && self.nb_ore_robots <= other.nb_ore_robots
            && self.nb_clay <= other.nb_clay
            && self.nb_clay_robots <= other.nb_clay_robots
            && self.nb_obsidian <= other.nb_obsidian
            && self.nb_obsidian_robots <= other.nb_obsidian_robots
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    nb_ore_for_ore: u32,
    nb_ore_for_clay: u32,
    nb_ore_clay_for_obsidian: (u32, u32),
    nb_ore_obsidian_for_geode: (u32, u32),
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
    fn quality_level(&self) -> u32 {
        self.id * self.nb_geodes(24)
    }
    fn next(&self, state: State) -> impl Iterator<Item = State> {
        self.make_geode_robot(state)
            .into_iter()
            .chain(self.make_obsidian_robot(state))
            .chain(self.make_clay_robot(state))
            .chain(self.make_ore_robot(state))
    }
    fn nb_geodes(&self, remaining: u32) -> u32 {
        let state = State {
            remaining,
            nb_ore_robots: 1,
            ..Default::default()
        };
        let mut q = std::collections::BinaryHeap::from(vec![state]);
        let mut solutions = vec![];
        while let Some(state) = q.pop() {
            if solutions.iter().any(|s| state.is_dominated_by(s)) {
                continue;
            }
            solutions.retain(|s| !s.is_dominated_by(&state));
            solutions.push(state);
            q.extend(self.next(state));
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
        .map(|l| Blueprint::try_from(l?.as_str()))
        .collect::<anyhow::Result<Vec<Blueprint>>>()?;

    let quality_levels: u32 = blueprints.iter().map(|b| b.quality_level()).sum();
    println!("Part1: {}", quality_levels);

    let product: u32 = blueprints.iter().take(3).map(|b| b.nb_geodes(32)).product();
    println!("Part2: {}", product);

    Ok(())
}
