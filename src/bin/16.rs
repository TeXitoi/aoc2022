use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
}

struct Room {
    rate: i32,
    is_open: bool,
    last_visit_releasing: i32,
    tunnels: HashMap<String, u32>,
}

#[derive(Default)]
struct Search {
    volcano: HashMap<String, Room>,
    best: i32,
}

impl Search {
    fn simplify(&mut self) {
        let zero_rate = self
            .volcano
            .iter()
            .filter_map(|(t, r)| (t != "AA" && r.rate == 0).then_some(t.clone()))
            .collect::<Vec<_>>();
        for zero_room_name in zero_rate {
            let zero_room = self.volcano.remove(&zero_room_name).unwrap();
            for (room_name, room) in &mut self.volcano {
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
    fn search(&mut self, room: &str, mut remaining: u32, mut releasing: i32) {
        if remaining == 0 {
            return;
        }

        let Some(cur) = self.volcano.get_mut(room) else { return };
        if cur.last_visit_releasing >= releasing {
            return;
        }
        let old_releasing = cur.last_visit_releasing;
        cur.last_visit_releasing = releasing;

        for (tunnel, &len) in cur.tunnels.clone().iter() {
            if len > remaining {
                continue;
            }
            self.search(tunnel, remaining - len, releasing);
        }

        let cur = self.volcano.get_mut(room).unwrap();
        let mut openned_here = false;
        if !cur.is_open && cur.rate > 0 {
            openned_here = true;
            cur.is_open = true;
            remaining -= 1;
            releasing += remaining as i32 * cur.rate;
            if self.best < releasing {
                self.best = releasing;
            }

            for (tunnel, &len) in cur.tunnels.clone().iter() {
                if len > remaining {
                    continue;
                }
                self.search(tunnel, remaining - len, releasing);
            }
        }

        let cur = self.volcano.get_mut(room).unwrap();
        cur.last_visit_releasing = old_releasing;
        if openned_here {
            cur.is_open = false;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut s = Search::default();
    for l in io::BufReader::new(std::fs::File::open("data/input16.txt")?).lines() {
        let l = l?;
        let Some(c) = RE.captures(&l) else { anyhow::bail!("bad line {:?}", l) };
        s.volcano.insert(
            String::from(&c[1]),
            Room {
                rate: c[2].parse()?,
                is_open: false,
                last_visit_releasing: -1,
                tunnels: c[3].split(", ").map(|t| (t.into(), 1)).collect(),
            },
        );
    }

    s.simplify();
    s.search("AA", 30, 0);
    println!("Part1: {}", s.best);

    Ok(())
}
