use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{self, BufRead};

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
}

struct Room {
    rate: u32,
    tunnels: HashMap<String, u32>,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    remaining: u32,
    releasing: u32,
    openned: HashSet<String>,
    position: String,
}
impl State {
    fn is_dominated_by(&self, other: &Self) -> bool {
        self.releasing <= other.releasing && self.openned.is_superset(&other.openned)
    }
    fn next<'a>(&'a self, volcano: &'a HashMap<String, Room>) -> impl Iterator<Item = State> + 'a {
        volcano[&self.position]
            .tunnels
            .iter()
            .filter(|(r, _)| !self.openned.contains(*r))
            .filter_map(|(room, &dist)| {
                let remaining = self.remaining.checked_sub(dist + 1)?;
                let mut openned = self.openned.clone();
                openned.insert(room.clone());
                Some(State {
                    remaining,
                    openned,
                    releasing: self.releasing + volcano[room].rate * remaining,
                    position: room.clone(),
                })
            })
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.remaining
            .cmp(&other.remaining)
            .then_with(|| self.releasing.cmp(&other.releasing))
    }
}

fn search(volcano: &HashMap<String, Room>, remaining: u32) -> impl Iterator<Item = State> {
    let mut states = HashMap::<_, Vec<_>>::new();
    let mut q = BinaryHeap::from(vec![State {
        remaining,
        releasing: 0,
        openned: Default::default(),
        position: "AA".into(),
    }]);
    while let Some(state) = q.pop() {
        let states = states.entry(state.position.clone()).or_default();
        if states.iter().any(|s| state.is_dominated_by(s)) {
            continue;
        }
        q.extend(state.next(volcano));
        states.retain(|s| !s.is_dominated_by(&state));
        states.push(state);
    }
    states.into_values().flatten()
}

fn best(states: impl Iterator<Item = State>) -> u32 {
    states.map(|s| s.releasing).max().unwrap_or(0)
}

fn best_at_2(states: impl Iterator<Item = State>) -> u32 {
    let mut solutions = Vec::<State>::new();
    for state in states {
        if solutions.iter().any(|s| state.is_dominated_by(s)) {
            continue;
        }
        solutions.retain(|s| !s.is_dominated_by(&state));
        solutions.push(state);
    }
    let mut releasing = 0;
    for (i, s1) in solutions.iter().enumerate() {
        for s2 in &solutions[..i] {
            if s1.openned.is_disjoint(&s2.openned) {
                releasing = releasing.max(s1.releasing + s2.releasing);
            }
        }
    }
    releasing
}

fn simplify(volcano: &mut HashMap<String, Room>) {
    // Floyd–Warshall
    let nodes: Vec<_> = volcano.keys().cloned().collect();
    for k in &nodes {
        for i in &nodes {
            let Some(&ik) = volcano.get(i).and_then(|r| r.tunnels.get(k)) else { continue };
            for j in &nodes {
                let Some(&kj) = volcano.get(k).and_then(|r| r.tunnels.get(j)) else { continue };
                let dist = volcano
                    .get_mut(i)
                    .unwrap()
                    .tunnels
                    .entry(j.clone())
                    .or_insert_with(|| u32::MAX);
                *dist = (ik + kj).min(*dist);
            }
        }
    }

    // remove useless nodes
    let targets: HashSet<_> = volcano
        .iter()
        .filter_map(|(k, v)| (v.rate > 0).then(|| k.clone()))
        .collect();
    volcano.retain(|k, _| targets.contains(k) || k == "AA");
    for room in volcano.values_mut() {
        room.tunnels.retain(|k, _| targets.contains(k));
    }
}

fn main() -> anyhow::Result<()> {
    let mut volcano = HashMap::new();
    for l in io::BufReader::new(std::fs::File::open("data/input16.txt")?).lines() {
        let l = l?;
        let Some(c) = RE.captures(&l) else { anyhow::bail!("bad line {:?}", l) };
        volcano.insert(
            c[1].into(),
            Room {
                rate: c[2].parse()?,
                tunnels: c[3].split(", ").map(|t| (t.into(), 1)).collect(),
            },
        );
    }
    simplify(&mut volcano);

    println!("Part1: {}", best(search(&volcano, 30)));
    println!("Part2: {}", best_at_2(search(&volcano, 26)));

    Ok(())
}
