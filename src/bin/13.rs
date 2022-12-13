use serde::Deserialize;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Msg {
    Int(u32),
    Vec(Vec<Msg>),
}
impl PartialOrd for Msg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Msg {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Msg::Int(l), Msg::Int(r)) => l.cmp(r),
            (Msg::Vec(l), Msg::Vec(r)) => l.cmp(r),
            (Msg::Vec(l), Msg::Int(_)) => l.as_slice().cmp(std::slice::from_ref(other)),
            (Msg::Int(_), Msg::Vec(r)) => std::slice::from_ref(self).cmp(r),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut score = 0;

    let first: Msg = serde_json::from_str("[[2]]")?;
    let second: Msg = serde_json::from_str("[[6]]")?;
    let mut msgs = vec![first.clone(), second.clone()];

    let mut iter = io::BufReader::new(std::fs::File::open("data/input13.txt")?).lines();
    for i in 1.. {
        let Some(l) = iter.next() else { break };
        let Some(r) = iter.next() else { anyhow::bail!("Unexpected end of stream") };

        let l = serde_json::from_str(&l?)?;
        let r = serde_json::from_str(&r?)?;
        if l <= r {
            score += i;
        }
        msgs.extend([l, r]);

        let Some(e) = iter.next().transpose()? else { break };
        if !e.is_empty() {
            anyhow::bail!("Unexpected non empty line {:?}", e);
        }
    }

    println!("Part1: {}", score);

    msgs.sort_unstable();
    let f_pos = msgs.iter().position(|m| m == &first).unwrap() + 1;
    let s_pos = msgs.iter().position(|m| m == &second).unwrap() + 1;
    println!("Part2: {}", f_pos * s_pos);

    Ok(())
}
