use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
}

struct Room {
    rate: i32,
    is_open: bool,
    last_visit_releasing: i32,
    tunnels: Arc<[String]>,
}

#[derive(Default)]
struct Search {
    volcano: HashMap<String, Room>,
    best: i32,
    best_path: Vec<String>,
    path: Vec<String>,
}

impl Search {
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

        if remaining > 0 {
            for tunnel in &*cur.tunnels.clone() {
                self.path.push(tunnel.clone());
                self.search(tunnel, remaining - 1, releasing);
                self.path.pop();
            }
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
                self.best_path = self.path.clone();
            }

            if remaining > 0 {
                remaining -= 1;
                for tunnel in &*cur.tunnels.clone() {
                    self.path.push(tunnel.clone());
                    self.search(tunnel, remaining, releasing);
                    self.path.pop();
                }
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
                tunnels: c[3].split(", ").map(String::from).collect(),
            },
        );
    }

    s.search("AA", 30, 0);
    println!("Part1: {}, {:?}", s.best, s.best_path);

    Ok(())
}
