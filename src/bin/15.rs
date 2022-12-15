use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};

lazy_static::lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

type C = i64;
const Y_TARGET: C = 2000000;
const SPACE: C = Y_TARGET * 2;

fn d((x1, y1): (C, C), (x2, y2): (C, C)) -> C {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn substract(set: &mut Vec<(C, C)>, s: (C, C)) {
    for i in std::mem::take(set) {
        if i.0 < s.0 && s.0 <= i.1 {
            set.push((i.0, s.0 - 1));
        }
        if i.0 <= s.1 && s.1 < i.1 {
            set.push((s.1 + 1, i.1));
        }
        if s.1 < i.0 || i.1 < s.0 {
            set.push(i);
        }
    }
}

fn range_at(s: (C, C), b: (C, C), at: C) -> std::ops::RangeInclusive<C> {
    let b_dist = d(s, b);
    let target_dist = d(s, (s.0, at));
    let y_scanned = b_dist - target_dist;
    s.0 - y_scanned..=s.0 + y_scanned
}

fn main() -> anyhow::Result<()> {
    let mut beacon = HashSet::new();
    let mut scanned = HashSet::new();
    let mut unscanned = vec![vec![(0, SPACE)]; SPACE as usize];

    for l in io::BufReader::new(std::fs::File::open("data/input15.txt")?).lines() {
        let l = l?;
        let Some(c) = RE.captures(&l) else { anyhow::bail!("bad line {:?}", l) };
        let (s, b): ((C, C), (C, C)) = (
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
                substract(v, r.into_inner());
            }
        }
    }

    println!("Part1: {}", scanned.len() - beacon.len());

    for (y, v) in unscanned.iter().enumerate() {
        for &(x1, x2) in v {
            for x in x1..=x2 {
                println!("Part2: {}", y as C + x * 4000000);
            }
        }
    }

    Ok(())
}
