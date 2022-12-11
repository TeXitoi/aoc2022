use std::io::{self, BufRead};
use std::sync::Arc;

type Score = u64;
type Operation = Arc<dyn Fn(Score) -> Score>;

#[derive(Clone)]
struct Monkey {
    items: Vec<Score>,
    operation: Operation,
    test: Score,
    if_true: usize,
    if_false: usize,
    nb_inspections: usize,
}
impl Monkey {
    fn try_new(mut iter: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<Self> {
        let mut items = Err(anyhow::anyhow!("no items"));
        let mut operation: Result<Operation, _> = Err(anyhow::anyhow!("no operaion"));
        let mut test = Err(anyhow::anyhow!("no test"));
        let mut if_true = Err(anyhow::anyhow!("no if_true"));
        let mut if_false = Err(anyhow::anyhow!("no if_false"));
        loop {
            let Some(l) = iter.next() else { break };
            let l = l?;
            match *l.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>() {
                [] => break,
                ["Monkey", _] => (),
                ["Starting", "items:", ref v @ ..] => {
                    items = Ok(v
                        .iter()
                        .map(|s| s.trim_end_matches(',').parse::<Score>())
                        .collect::<Result<_, _>>()?);
                }
                ["Operation:", "new", "=", "old", "*", "old"] => {
                    operation = Ok(Arc::new(move |i| i * i));
                }
                ["Operation:", "new", "=", "old", "*", nb] => {
                    let nb = nb.parse::<Score>()?;
                    operation = Ok(Arc::new(move |i| i * nb));
                }
                ["Operation:", "new", "=", "old", "+", nb] => {
                    let nb = nb.parse::<Score>()?;
                    operation = Ok(Arc::new(move |i| i + nb));
                }
                ["Test:", "divisible", "by", nb] => test = Ok(nb.parse()?),
                ["If", "true:", "throw", "to", "monkey", nb] => if_true = Ok(nb.parse()?),
                ["If", "false:", "throw", "to", "monkey", nb] => if_false = Ok(nb.parse()?),
                _ => anyhow::bail!("unknown line {:?}", l),
            }
        }
        Ok(Self {
            items: items?,
            operation: operation?,
            test: test?,
            if_true: if_true?,
            if_false: if_false?,
            nb_inspections: 0,
        })
    }

    fn turn(&mut self, manage: impl Fn(Score) -> Score) -> Vec<(usize, Score)> {
        let items = std::mem::take(&mut self.items);
        let v: Vec<_> = items
            .into_iter()
            .map(|i| {
                let i = (self.operation)(i);
                let i = manage(i);
                let to_send = if i % self.test == 0 {
                    self.if_true
                } else {
                    self.if_false
                };
                (to_send, i)
            })
            .collect();
        self.nb_inspections += v.len();
        v
    }
}

fn run(mut monkeys: Vec<Monkey>, nb: u32, manage: impl Fn(Score) -> Score) -> usize {
    for _ in 0..nb {
        for m in 0..monkeys.len() {
            for (m, i) in monkeys[m].turn(&manage) {
                monkeys[m].items.push(i);
            }
        }
    }
    monkeys.sort_by_key(|m| m.nb_inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.nb_inspections)
        .product()
}

fn main() -> anyhow::Result<()> {
    let mut lines = io::BufReader::new(std::fs::File::open("data/input11.txt")?)
        .lines()
        .peekable();
    let mut monkeys = vec![];
    while lines.peek().is_some() {
        monkeys.push(Monkey::try_new(&mut lines)?);
    }

    println!("Part1: {}", run(monkeys.clone(), 20, |s| s / 3),);

    let modulus: Score = monkeys.iter().map(|m| m.test).product();
    println!("modulus: {}", modulus);
    println!("Part2: {}", run(monkeys, 10000, |s| s % modulus),);

    Ok(())
}
