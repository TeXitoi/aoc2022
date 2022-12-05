use std::io::BufRead;

fn is_crate(c: u8) -> bool {
    b'A' <= c && c <= b'Z'
}

fn create_stack(
    iter: impl Iterator<Item = std::io::Result<String>>,
) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut res = vec![];
    for l in iter {
        let mut l = l?.into_bytes();
        let mut idx = 0;
        l.retain(|_| {
            idx += 1;
            idx % 4 == 2
        });
        res.resize(l.len(), vec![]);
        for (i, c) in l.into_iter().enumerate() {
            if is_crate(c) {
                res[i].push(c);
            }
        }
    }
    for v in &mut res {
        v.reverse();
    }
    Ok(res)
}

fn parse_move(s: &str) -> anyhow::Result<(usize, usize, usize)> {
    let v = s
        .split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    match v.as_slice() {
        &[nb, from, to] => Ok((nb, from - 1, to - 1)),
        _ => anyhow::bail!("error parsing move"),
    }
}

fn make_move_9000(s: &mut [Vec<u8>], (nb, from, to): (usize, usize, usize)) {
    for _ in 0..nb {
        let Some(c) = s[from].pop() else { return };
        s[to].push(c);
    }
}

fn make_move_9001(s: &mut [Vec<u8>], m @ (nb, _, to): (usize, usize, usize)) {
    make_move_9000(s, m);
    let v = &mut s[to];
    let len = v.len();
    v[len - nb..].reverse();
}

fn run(f: fn(&mut [Vec<u8>], (usize, usize, usize))) -> anyhow::Result<String> {
    let mut lines = std::io::BufReader::new(std::fs::File::open("data/input5.txt")?).lines();
    let mut stack = create_stack(
        lines
            .by_ref()
            .take_while(|l| l.as_ref().map_or(false, |l| !l.is_empty())),
    )?;

    for m in lines {
        let m = parse_move(&m?)?;
        f(&mut stack, m);
    }

    let res: String = stack
        .iter()
        .map(|v| v.last().copied().unwrap_or(b' ') as char)
        .collect();

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    println!("part1: {}", run(make_move_9000)?);
    println!("part2: {}", run(make_move_9001)?);

    Ok(())
}
