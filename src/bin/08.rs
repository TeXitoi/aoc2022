use std::io::{self, BufRead};

fn look(h: &[Vec<u8>], v: &mut [Vec<bool>], iter: impl Iterator<Item = (usize, usize)>) {
    let mut cur = 0;
    for (i, j) in iter {
        let h = h[i][j];
        if h > cur {
            v[i][j] = true;
        }
        cur = cur.max(h);
    }
}

fn count(h: u8, iter: impl Iterator<Item = u8>) -> usize {
    let mut nb = 0;
    for (i, cur_h) in iter.enumerate() {
        if cur_h >= h {
            return i + 1;
        }
        nb = i + 1;
    }
    nb
}

fn main() -> anyhow::Result<()> {
    let f = io::BufReader::new(std::fs::File::open("data/input8.txt")?)
        .lines()
        .map(|s| s.map(String::into_bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let len = f.len();

    let mut v = vec![vec![false; len]; len];
    for i in 0..len {
        look(&f, &mut v, (0..len).map(|j| (i, j)));
        look(&f, &mut v, (0..len).rev().map(|j| (i, j)));
        look(&f, &mut v, (0..len).map(|j| (j, i)));
        look(&f, &mut v, (0..len).rev().map(|j| (j, i)));
    }
    let nb = v.into_iter().flatten().filter(|v| *v).count();
    println!("Part1: {}", nb);

    let mut max = 0;
    for i in 0..len {
        for j in 0..len {
            let cur = count(f[i][j], (j + 1..len).map(|k| f[i][k]))
                * count(f[i][j], (i + 1..len).map(|k| f[k][j]))
                * count(f[i][j], (0..j).rev().map(|k| f[i][k]))
                * count(f[i][j], (0..i).rev().map(|k| f[k][j]));
            max = max.max(cur);
        }
    }
    println!("Part2: {}", max);

    Ok(())
}
