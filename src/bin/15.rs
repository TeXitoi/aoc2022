use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

type C = i64;
type Range = std::ops::RangeInclusive<C>;
const Y_TARGET: C = 2000000;
const SPACE: C = Y_TARGET * 2;

fn d(a: (C, C), b: (C, C)) -> C {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn substract(set: &mut Vec<Range>, s: Range) {
    for r in std::mem::take(set) {
        if r.start() < s.start() && s.start() <= r.end() {
            set.push(*r.start()..=*s.start() - 1);
        }
        if r.start() <= s.end() && s.end() < r.end() {
            set.push(*s.end() + 1..=*r.end());
        }
        if s.end() < r.start() || r.end() < s.start() {
            set.push(r);
        }
    }
}

fn range_at(s: (C, C), b: (C, C), at: C) -> Range {
    let b_dist = d(s, b);
    let target_dist = d(s, (s.0, at));
    let y_scanned = b_dist - target_dist;
    s.0 - y_scanned..=s.0 + y_scanned
}

fn main() -> anyhow::Result<()> {
    let mut beacon = HashSet::new();
    let mut scanned = HashSet::new();
    let mut unscanned = vec![vec![0..=SPACE]; SPACE as usize + 1];

    for l in io::BufReader::new(std::fs::File::open("data/input15.txt")?).lines() {
        let l = l?;
        let Some(c) = RE.captures(&l) else { anyhow::bail!("bad line {:?}", l) };
        let (s, b) = (
            (c[1].parse()?, c[2].parse()?),
            (c[3].parse()?, c[4].parse()?),
        );

        if b.1 == Y_TARGET {
            beacon.insert(b.0);
        }
        for x in range_at(s, b, Y_TARGET) {
            scanned.insert(x);
        }

        for (i, v) in unscanned.iter_mut().enumerate() {
            let r = range_at(s, b, i as C);
            if !r.is_empty() {
                substract(v, r);
            }
        }
    }

    println!("Part1: {}", scanned.len() - beacon.len());

    for (y, v) in unscanned.into_iter().enumerate() {
        for r in v {
            for x in r {
                println!("Part2: {}", y as C + x * 4000000);
            }
        }
    }

    Ok(())
}
