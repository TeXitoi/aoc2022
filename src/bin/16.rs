use regex::Regex;
use smol_str::SmolStr as String;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
}

#[derive(Clone)]
struct Room {
    rate: i32,
    tunnels: HashMap<String, u32>,
}

#[derive(Default)]
struct Search {
    non_dominated: Vec<Arc<State>>,
    q: std::collections::BinaryHeap<Arc<State>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    fn push(&mut self, state: State) {
        if self.non_dominated.iter().any(|s| state.is_dominated_by(s)) {
            return;
        }
        let state = Arc::new(state);
        if state.remaining != 0 {
            self.q.push(state.clone());
        }
        self.non_dominated.push(state);
    }
    fn search(volcano: &HashMap<String, Room>, remaining: u32) -> Self {
        let mut s = Self::default();
        s.push(State {
            remaining,
            releasing: 0,
            openned: Default::default(),
            position: "AA".into(),
        });
        while let Some(cur_state) = s.q.pop() {
            let room = volcano.get(&cur_state.position).unwrap();
            if room.rate > 0 && !cur_state.openned.contains(&cur_state.position) {
                let mut openned = cur_state.openned.clone();
                openned.insert(cur_state.position.clone());
                s.push(State {
                    remaining: cur_state.remaining - 1,
                    releasing: cur_state.releasing + room.rate * (cur_state.remaining as i32 - 1),
                    openned,
                    position: cur_state.position.clone(),
                });
            }
            for (room, &dist) in &room.tunnels {
                if dist > cur_state.remaining {
                    continue;
                }
                s.push(State {
                    remaining: cur_state.remaining - dist,
                    releasing: cur_state.releasing,
                    openned: cur_state.openned.clone(),
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
}

fn simplify(volcano: &mut HashMap<String, Room>) {
    let zero_rate = volcano
        .iter()
        .filter_map(|(t, r)| (t != "AA" && r.rate == 0).then_some(t.clone()))
        .collect::<Vec<_>>();
    for zero_room_name in zero_rate {
        let zero_room = volcano.remove(&zero_room_name).unwrap();
        for (room_name, room) in volcano.iter_mut() {
            let Some(len) = room.tunnels.remove(&zero_room_name) else { continue };
            for (tunnel, additional_len) in &zero_room.tunnels {
                if tunnel == room_name {
                    continue;
                }
                let cur_len = room
                    .tunnels
                    .entry(tunnel.clone())
                    .or_insert_with(|| u32::MAX);
                *cur_len = (*cur_len).min(len + additional_len);
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
    let s = Search::search(&volcano, 30);
    println!("Part1: {}", s.best());

    let s = Search::search(&volcano, 26);
    let mut releasing = 0;
    for (i, s1) in s.non_dominated.iter().enumerate() {
        for s2 in &s.non_dominated[0..i] {
            if s1.openned.is_disjoint(&s2.openned) {
                releasing = releasing.max(s1.releasing + s2.releasing);
            }
        }
    }
    println!("Part1: {}", releasing);

    Ok(())
}
