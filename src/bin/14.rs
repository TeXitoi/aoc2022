use std::io::{self, BufRead};

const S_INIT: (usize, usize) = (500, 0);

macro_rules! check {
    ($is_air:ident, $s:ident, $n:expr) => {
        let Some(&can) = $is_air.get($n.0).and_then(|v| v.get($n.1)) else { break };
        if can {
            $s = $n;
            continue;
        }
    };
}

fn simulate(mut is_air: Vec<Vec<bool>>) -> usize {
    let mut i = 0;
    let mut s = S_INIT;
    loop {
        check!(is_air, s, (s.0, s.1 + 1));
        check!(is_air, s, (s.0 - 1, s.1 + 1));
        check!(is_air, s, (s.0 + 1, s.1 + 1));
        i += 1;
        if s == S_INIT {
            break;
        }
        is_air[s.0][s.1] = false;
        s = S_INIT;
    }
    i
}

fn main() -> anyhow::Result<()> {
    let mut is_air: Vec<Vec<bool>> = vec![];
    for l in io::BufReader::new(std::fs::File::open("data/input14.txt")?).lines() {
        let l = l?;
        let coords = l
            .split(" -> ")
            .map(|s| {
                let (i, j) = s
                    .split_once(',')
                    .ok_or_else(|| anyhow::anyhow!("no comma"))?;
                anyhow::Ok((i.parse::<usize>()?, j.parse::<usize>()?))
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (&a, &b) in coords.iter().zip(&coords[1..]) {
            for x in a.0.min(b.0)..=a.0.max(b.0) {
                for y in a.1.min(b.1)..=a.1.max(b.1) {
                    is_air.resize(is_air.len().max(x + 1), vec![]);
                    let is_air_x_len = is_air[x].len();
                    is_air[x].resize(is_air_x_len.max(y + 1), true);
                    is_air[x][y] = false;
                }
            }
        }
    }

    println!("Part1: {}", simulate(is_air.clone()));

    let y_floor = is_air.iter().map(Vec::len).max().unwrap_or(0) + 2;
    is_air.resize(y_floor + S_INIT.0 + 1, vec![]);
    for v in &mut is_air {
        v.resize(y_floor, true);
        v[y_floor - 1] = false;
    }
    println!("Part2: {}", simulate(is_air));

    Ok(())
}
