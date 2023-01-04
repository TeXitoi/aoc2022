use std::cmp::Ordering::*;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut r = [(0, 0); 10];
    let mut s1 = HashSet::new();
    let mut s9 = HashSet::new();

    for l in io::BufReader::new(std::fs::File::open("data/input9.txt")?).lines() {
        let l = l?;
        let (m, nb) = match *l.split(' ').collect::<Vec<_>>() {
            [m, nb] => (m, nb.parse()?),
            _ => anyhow::bail!("bad line {:?}", l),
        };
        for _ in 0..nb {
            match m {
                "U" => r[0].1 += 1,
                "D" => r[0].1 -= 1,
                "L" => r[0].0 -= 1,
                "R" => r[0].0 += 1,
                _ => anyhow::bail!("bad move {:?}", m),
            }
            for i in 0..r.len() - 1 {
                let (h, t) = (r[i], &mut r[i + 1]);
                if (h.0 - 1..=h.0 + 1).contains(&t.0) && (h.1 - 1..=h.1 + 1).contains(&t.1) {
                    continue;
                }
                match h.0.cmp(&t.0) {
                    Less => t.0 -= 1,
                    Greater => t.0 += 1,
                    Equal => (),
                }
                match h.1.cmp(&t.1) {
                    Less => t.1 -= 1,
                    Greater => t.1 += 1,
                    Equal => (),
                }
            }

            s1.insert(r[1]);
            s9.insert(r[9]);
        }
    }

    println!("Part1: {}", s1.len());
    println!("Part2: {}", s9.len());

    Ok(())
}
