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

fn update(map: &[Vec<u8>], distances: &mut [Vec<u32>], i: usize, j: usize) -> bool {
    [
        (i.checked_sub(1), Some(j)),
        (Some(i), j.checked_sub(1)),
        (Some(i), j.checked_add(1)),
        (i.checked_add(1), Some(j)),
    ]
    .into_iter()
    .filter_map(|(cur_i, cur_j)| {
        let (cur_i, cur_j) = (cur_i?, cur_j?);
        (*map.get(cur_i)?.get(cur_j)? <= map[i][j] + 1)
            .then(|| distances[cur_i][cur_j])?
            .checked_add(1)
    })
    .min()
    .filter(|d| *d < distances[i][j])
    .map(|d| distances[i][j] = d)
    .is_some()
}

fn main() -> anyhow::Result<()> {
    let mut map = io::BufReader::new(std::fs::File::open("data/input12.txt")?)
        .lines()
        .map(|s| s.map(String::into_bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let source = find_and_replace(&mut map, b'S', b'a')?;
    let target = find_and_replace(&mut map, b'E', b'z')?;
    let map = &map;
    let mut distances: Vec<_> = map.iter().map(|v| vec![u32::MAX; v.len()]).collect();
    distances[target.0][target.1] = 0;

    // Some naive shortest path because I'm lazy to implement Dijkstra. O(size×longest_path)
    while (0..map.len())
        .flat_map(|i| (0..map[i].len()).map(move |j| (i, j)))
        .any(|(i, j)| update(map, &mut distances, i, j))
    {}

    println!("Part1: {}", distances[source.0][source.1]);

    let min = distances
        .iter()
        .enumerate()
        .flat_map(|(i, d)| {
            d.iter()
                .enumerate()
                .filter_map(move |(j, d)| (map[i][j] == b'a').then_some(d))
        })
        .min();
    println!("Part2: {}", min.unwrap_or(&u32::MAX));

    Ok(())
}
