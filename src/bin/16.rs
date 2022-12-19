use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
}

struct Room {
    rate: i32,
    tunnels: HashMap<String, u32>,
}

#[derive(Default)]
struct Search {
    non_dominated: Vec<State>,
    q: std::collections::BinaryHeap<State>,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    remaining: u32,
    releasing: i32,
    openned: HashSet<String>,
    position: String,
}
impl State {
    fn is_dominated_by(&self, other: &Self) -> bool {
        if self.position != other.position {
            return false;
        }
        self.remaining <= other.remaining
            && self.releasing <= other.releasing
            && self.openned.is_superset(&other.openned)
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

impl Search {
    fn run(volcano: &HashMap<String, Room>, remaining: u32) -> Self {
        let mut s = Self::default();
        let targets: HashSet<_> = volcano
            .iter()
            .filter_map(|(k, v)| (v.rate > 0).then_some(k.clone()))
            .collect();
        s.q.push(State {
            remaining,
            releasing: 0,
            openned: Default::default(),
            position: "AA".into(),
        });
        while let Some(cur_state) = s.q.pop() {
            if s.non_dominated.iter().any(|s| cur_state.is_dominated_by(s)) {
                continue;
            }
            s.non_dominated.push(cur_state.clone());
            let room = volcano.get(&cur_state.position).unwrap();
            for (room, &dist) in &room.tunnels {
                if !targets.contains(room) {
                    continue;
                }
                if cur_state.openned.contains(room) {
                    continue;
                }
                if dist + 1 > cur_state.remaining {
                    continue;
                }
                let Some(rate) = volcano.get(room).map(|r| r.rate) else { continue };
                let remaining = cur_state.remaining - dist - 1;
                let mut openned = cur_state.openned.clone();
                openned.insert(room.clone());
                s.q.push(State {
                    remaining,
                    openned,
                    releasing: cur_state.releasing + rate * (remaining as i32),
                    position: room.clone(),
                });
            }
        }
        s
    }
    fn best(&self) -> i32 {
        self.non_dominated
            .iter()
            .map(|s| s.releasing)
            .max()
            .unwrap_or(0)
    }
    fn best_at_2(&self) -> i32 {
        let mut releasing = 0;
        for (i, s1) in self.non_dominated.iter().enumerate() {
            for s2 in &self.non_dominated[0..i] {
                if s1.openned.is_disjoint(&s2.openned) {
                    releasing = releasing.max(s1.releasing + s2.releasing);
                }
            }
        }
        releasing
    }
}

fn simplify(volcano: &mut HashMap<String, Room>) {
    let nodes: Vec<_> = volcano.keys().cloned().collect();
    for k in &nodes {
        for i in &nodes {
            let Some(&ik) = volcano.get(i).and_then(|r| r.tunnels.get(k)) else { continue };
            for j in &nodes {
                let Some(&kj) = volcano.get(k).and_then(|r| r.tunnels.get(j)) else { continue };
                let new_dist = ik + kj;
                let dist = volcano
                    .get_mut(i)
                    .unwrap()
                    .tunnels
                    .entry(j.clone())
                    .or_insert_with(|| u32::MAX);
                *dist = new_dist.min(*dist);
            }
        }
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

    let s = Search::run(&volcano, 30);
    println!("Part1: {}", s.best());

    let s = Search::run(&volcano, 26);
    println!("Part2: {}", s.best_at_2());

    Ok(())
}
