use std::io::{self, BufRead};

fn find_and_replace(
    map: &mut [Vec<u8>],
    target: u8,
    replace: u8,
) -> anyhow::Result<(usize, usize)> {
    for (i, v) in map.iter_mut().enumerate() {
        for (j, c) in v.iter_mut().enumerate() {
            if *c == target {
                *c = replace;
                return Ok((i, j));
            }
        }
    }
    anyhow::bail!("Not found")
}

fn update<'a>(
    map: &'a [Vec<u8>],
    dists: &'a mut [Vec<u32>],
    (i, j): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let d = dists[i][j] + 1;
    [
        (i.checked_sub(1), Some(j)),
        (Some(i), j.checked_sub(1)),
        (Some(i), j.checked_add(1)),
        (i.checked_add(1), Some(j)),
    ]
    .into_iter()
    .filter_map(move |(cur_i, cur_j)| {
        let (cur_i, cur_j) = (cur_i?, cur_j?);
        if *map.get(cur_i)?.get(cur_j)? + 1 >= map[i][j] && d < dists[cur_i][cur_j] {
            dists[cur_i][cur_j] = d;
            Some((cur_i, cur_j))
        } else {
            None
        }
    })
}

fn main() -> anyhow::Result<()> {
    let mut map = io::BufReader::new(std::fs::File::open("data/input12.txt")?)
        .lines()
        .map(|s| s.map(String::into_bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let source = find_and_replace(&mut map, b'S', b'a')?;
    let target = find_and_replace(&mut map, b'E', b'z')?;
    let map = &map;
    let mut dists: Vec<_> = map.iter().map(|v| vec![u32::MAX; v.len()]).collect();
    dists[target.0][target.1] = 0;

    // BFS as distance is always 1
    let mut q = std::collections::VecDeque::from_iter([target]);
    while let Some(p) = q.pop_front() {
        q.extend(update(map, &mut dists, p));
    }

    println!("Part1: {}", dists[source.0][source.1]);

    let min = dists
        .iter()
        .zip(map)
        .flat_map(|(d, m)| d.iter().zip(m))
        .filter_map(|(&d, &m)| (m == b'a').then_some(d))
        .min();
    println!("Part2: {}", min.unwrap_or(u32::MAX));

    Ok(())
}
