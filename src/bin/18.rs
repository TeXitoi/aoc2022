use std::collections::HashSet;
use std::io::{self, BufRead};

type Cube = [i8; 3];
fn next(min: Cube, max: Cube, cur: Cube) -> impl Iterator<Item = Cube> {
    [
        [cur[0] + 1, cur[1], cur[2]],
        [cur[0] - 1, cur[1], cur[2]],
        [cur[0], cur[1] + 1, cur[2]],
        [cur[0], cur[1] - 1, cur[2]],
        [cur[0], cur[1], cur[2] + 1],
        [cur[0], cur[1], cur[2] - 1],
    ]
    .into_iter()
    .filter(move |c| c.iter().zip(min).all(|(&c, m)| m <= c))
    .filter(move |c| c.iter().zip(max).all(|(&c, m)| c <= m))
}

fn main() -> anyhow::Result<()> {
    let mut cubes = HashSet::<Cube>::new();
    for l in io::BufReader::new(std::fs::File::open("data/input18.txt")?).lines() {
        cubes.insert(
            l?.split(',')
                .map(|c| c.parse())
                .collect::<Result<Vec<_>, _>>()?
                .as_slice()
                .try_into()?,
        );
    }

    let mut nb_faces = cubes.len() * 6;
    let mut iter = cubes.iter();
    while let Some(&c1) = iter.next() {
        for &c2 in iter.clone() {
            if c1
                .into_iter()
                .zip(c2)
                .map(|(c1, c2)| (c1 - c2).abs())
                .sum::<i8>()
                == 1
            {
                nb_faces -= 2;
            }
        }
    }
    println!("Part1: {}", nb_faces);

    let mut min = *cubes.iter().next().unwrap_or(&[0, 0, 0]);
    let mut max = min;
    for &c in &cubes {
        for i in 0..3 {
            min[i] = min[i].min(c[i] - 1);
            max[i] = max[i].max(c[i] + 1);
        }
    }
    let mut visited = HashSet::new();
    let mut q = vec![min];
    let mut nb_faces = 0;
    while let Some(cur) = q.pop() {
        for c in next(min, max, cur) {
            if visited.contains(&c) {
                continue;
            }
            if cubes.contains(&c) {
                nb_faces += 1;
            } else {
                visited.insert(c);
                q.push(c);
            }
        }
    }
    println!("Part2: {}", nb_faces);

    Ok(())
}
