use std::io::BufRead;

#[derive(Copy, Clone)]
struct Move {
    nb: usize,
    from: usize,
    to: usize,
}

fn is_crate(c: u8) -> bool {
    (b'A'..=b'Z').contains(&c)
}

fn create_stack(
    iter: impl Iterator<Item = std::io::Result<String>>,
) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut res = vec![];
    for l in iter {
        let mut l = l?.into_bytes();
        if l.is_empty() {
            break;
        }
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

impl std::str::FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Move> {
        let v = s
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        match *v {
            [nb, from, to] => Ok(Move {
                nb,
                from: from - 1,
                to: to - 1,
            }),
            _ => anyhow::bail!("error parsing move"),
        }
    }
}

fn make_move_9000(s: &mut [Vec<u8>], m: Move) {
    for _ in 0..m.nb {
        let Some(c) = s[m.from].pop() else { return };
        s[m.to].push(c);
    }
}

fn make_move_9001(s: &mut [Vec<u8>], m: Move) {
    make_move_9000(s, m);
    let v = &mut s[m.to];
    let len = v.len();
    v[len - m.nb..].reverse();
}

fn run(f: fn(&mut [Vec<u8>], Move)) -> anyhow::Result<String> {
    let mut lines = std::io::BufReader::new(std::fs::File::open("data/input5.txt")?).lines();
    let mut stack = create_stack(lines.by_ref())?;

    for m in lines {
        f(&mut stack, m?.parse()?);
    }

    let res: String = stack
        .iter()
        .map(|v| v.last().copied().unwrap_or(b' ') as char)
        .collect();

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    println!("Part1: {}", run(make_move_9000)?);
    println!("Part2: {}", run(make_move_9001)?);

    Ok(())
}
