use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Monkey {
    Val(i64),
    Op {
        lhs: String,
        rhs: String,
        op: String,
    },
    Humn(i64),
}
impl Monkey {
    fn try_new(s: &str) -> anyhow::Result<(String, Monkey)> {
        match *s.split(' ').collect::<Vec<_>>() {
            ["humn:", val] => Ok(("humn".into(), Monkey::Humn(val.parse()?))),
            [name, val] => Ok((name.trim_end_matches(':').into(), Monkey::Val(val.parse()?))),
            [name, lhs, op, rhs] => Ok((
                name.trim_end_matches(':').into(),
                Monkey::Op {
                    lhs: lhs.into(),
                    rhs: rhs.into(),
                    op: op.into(),
                },
            )),
            _ => anyhow::bail!("bad line {:?}", s),
        }
    }
    fn val(&self, take_humn: bool) -> Option<i64> {
        match *self {
            Monkey::Val(v) => Some(v),
            Monkey::Humn(v) if take_humn => Some(v),
            _ => None,
        }
    }
}

fn propagate(monkeys: &mut HashMap<String, Monkey>, take_humn: bool) -> anyhow::Result<()> {
    let keys = monkeys.keys().cloned().collect::<Vec<_>>();
    let mut update = true;
    while update {
        update = false;
        for key in &keys {
            let val = match &monkeys[key] {
                Monkey::Val(_) | Monkey::Humn(_) => continue,
                Monkey::Op { lhs, rhs, op } => {
                    let Some(lhs) = monkeys.get(lhs).and_then(|m| m.val(take_humn)) else { continue; };
                    let Some(rhs) = monkeys.get(rhs).and_then(|m| m.val(take_humn)) else { continue; };
                    match op.as_str() {
                        "*" => lhs * rhs,
                        "+" => lhs + rhs,
                        "/" => lhs / rhs,
                        "-" => lhs - rhs,
                        _ => anyhow::bail!("bad op {:?}", op),
                    }
                }
            };
            monkeys.insert(key.clone(), Monkey::Val(val));
            update = true;
        }
    }
    Ok(())
}

fn inverse(monkeys: &HashMap<String, Monkey>, cur: &Monkey, equal: i64) -> anyhow::Result<i64> {
    use Monkey::*;
    match cur {
        Humn(_) => Ok(equal),
        Op { lhs, rhs, op } => match (&monkeys[lhs], op.as_str(), &monkeys[rhs]) {
            (&Val(m), "+", other) | (other, "+", &Val(m)) => inverse(monkeys, other, equal - m),
            (&Val(m), "*", other) | (other, "*", &Val(m)) => inverse(monkeys, other, equal / m),
            (other, "-", &Val(m)) => inverse(monkeys, other, equal + m),
            (&Val(m), "-", other) => inverse(monkeys, other, m - equal),
            (other, "/", &Val(m)) => inverse(monkeys, other, equal * m),
            (&Val(m), "/", other) => inverse(monkeys, other, m / equal),
            v => anyhow::bail!("Unsupported inversion {:?}", v),
        },
        _ => anyhow::bail!("Unsupported node {:?}", cur),
    }
}

fn main() -> anyhow::Result<()> {
    let monkeys_orig = io::BufReader::new(std::fs::File::open("data/input21.txt")?)
        .lines()
        .map(|l| Monkey::try_new(&l?))
        .collect::<anyhow::Result<HashMap<_, _>>>()?;

    let mut monkeys = monkeys_orig.clone();
    propagate(&mut monkeys, true)?;
    println!(
        "Part1: {}",
        monkeys
            .get("root")
            .and_then(|m| m.val(true))
            .ok_or_else(|| anyhow::anyhow!("can't compute root"))?,
    );

    let mut monkeys = monkeys_orig;
    match monkeys.get_mut("root") {
        Some(Monkey::Op { op, .. }) => *op = "-".into(),
        root => anyhow::bail!("unsupported root {:?}", root),
    }
    propagate(&mut monkeys, false)?;
    println!("Part2: {}", inverse(&monkeys, &monkeys["root"], 0)?);

    Ok(())
}
