use std::io::{self, BufRead};

fn mix(v: &mut [(usize, i64)]) {
    let len = v.len();
    for i in 0..len {
        let (pos, &(_, value)) = v.iter().enumerate().find(|(_, &(j, _))| i == j).unwrap();
        let modulus = len as i64 - 1;
        let mut new_pos = (pos as i64 + value) % modulus;
        if new_pos < 0 {
            new_pos += modulus;
        }
        let new_pos = new_pos as usize;
        if pos < new_pos {
            v[pos..=new_pos].rotate_left(1);
        } else {
            v[new_pos..=pos].rotate_right(1);
        }
    }
}

fn grove(v: &[(usize, i64)]) -> i64 {
    let len = v.len();
    let p = v.iter().position(|(_, v)| *v == 0).unwrap();
    v[(p + 1000) % len].1 + v[(p + 2000) % len].1 + v[(p + 3000) % len].1
}

fn main() -> anyhow::Result<()> {
    let v_orig = io::BufReader::new(std::fs::File::open("data/input20.txt")?)
        .lines()
        .enumerate()
        .map(|(i, v)| Ok((i, v?.parse()?)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut v = v_orig.clone();
    mix(&mut v);
    println!("Part1: {}", grove(&v));

    let mut v = v_orig.clone();
    for (_, v) in &mut v {
        *v *= 811589153;
    }
    for _ in 0..10 {
        mix(&mut v);
    }
    println!("Part2: {}", grove(&v));

    Ok(())
}
