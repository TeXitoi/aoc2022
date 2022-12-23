use std::collections::HashMap;
use std::io::{self, BufRead};

type Coord = (usize, usize);

fn next(map: &[Vec<bool>], coord: Coord, iteration: usize) -> Option<Coord> {
    let nothing_around = [
        (coord.0 - 1, coord.1 - 1),
        (coord.0 - 1, coord.1),
        (coord.0 - 1, coord.1 + 1),
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 + 1),
        (coord.0 + 1, coord.1 - 1),
        (coord.0 + 1, coord.1),
        (coord.0 + 1, coord.1 + 1),
    ]
    .into_iter()
    .all(|c| !map[c.0][c.1]);
    if nothing_around {
        return None;
    }
    [
        [
            (coord.0 - 1, coord.1 - 1),
            (coord.0 - 1, coord.1),
            (coord.0 - 1, coord.1 + 1),
        ],
        [
            (coord.0 + 1, coord.1 - 1),
            (coord.0 + 1, coord.1),
            (coord.0 + 1, coord.1 + 1),
        ],
        [
            (coord.0 - 1, coord.1 - 1),
            (coord.0, coord.1 - 1),
            (coord.0 + 1, coord.1 - 1),
        ],
        [
            (coord.0 - 1, coord.1 + 1),
            (coord.0, coord.1 + 1),
            (coord.0 + 1, coord.1 + 1),
        ],
    ]
    .into_iter()
    .cycle()
    .skip(iteration % 4)
    .take(4)
    .filter_map(|l @ [_, d, _]| l.into_iter().all(|c| !map[c.0][c.1]).then_some(d))
    .next()
}

fn extend_map(map: &mut Vec<Vec<bool>>) {
    if map.is_empty() {
        return;
    }
    let len = map[0].len();
    if map[0].iter().any(|c| *c) {
        map.insert(0, vec![false; len]);
    }
    if map[map.len() - 1].iter().any(|c| *c) {
        map.push(vec![false; len]);
    }
    if map.iter().any(|v| v[0]) {
        for v in map.iter_mut() {
            v.insert(0, false);
        }
    }
    if map.iter().any(|v| v[v.len() - 1]) {
        for v in map {
            v.push(false);
        }
    }
}

fn shrink_map(map: &mut Vec<Vec<bool>>) {
    while map[map.len() - 1].iter().all(|c| !*c) {
        map.pop();
    }
    while map[0].iter().all(|c| !*c) {
        map.remove(0);
    }
    while map.iter().all(|v| !v[v.len() - 1]) {
        for v in map.iter_mut() {
            v.pop();
        }
    }
    while map.iter().all(|v| !v[0]) {
        for v in map.iter_mut() {
            v.remove(0);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut map = io::BufReader::new(std::fs::File::open("data/input23.txt")?)
        .lines()
        .map(|l| Ok(l?.as_bytes().iter().map(|&c| c == b'#').collect()))
        .collect::<anyhow::Result<Vec<Vec<_>>>>()?;

    for k in 0.. {
        extend_map(&mut map);
        let prev = map.clone();

        let mut propositions = HashMap::<_, Vec<Coord>>::new();
        for (i, v) in map.iter().enumerate() {
            for (j, _) in v.iter().enumerate().filter(|(_, &e)| e) {
                if let Some(d) = next(&map, (i, j), k) {
                    propositions.entry(d).or_default().push((i, j));
                }
            }
        }
        for (d, es) in propositions {
            if es.len() == 1 {
                map[d.0][d.1] = true;
                map[es[0].0][es[0].1] = false;
            }
        }

        if k == 9 {
            shrink_map(&mut map);
            println!("Part1: {}", map.iter().flatten().filter(|&&c| !c).count());
        }
        if map == prev {
            println!("Part2: {}", k + 1);
            break;
        }
    }

    Ok(())
}
