use std::cmp::Ordering;
use std::io::{self, BufRead};

fn peek(s: &[u8]) -> &[u8] {
    if s.is_empty() {
        return s;
    }
    let i = 1.max(s.iter().take_while(|c| (b'0'..=b'9').contains(c)).count());
    &s[0..i]
}

fn take<'a>(s: &mut &'a [u8]) -> &'a [u8] {
    let t = peek(s);
    *s = &s[t.len()..];
    t
}

fn parse(s: &mut &[u8]) -> anyhow::Result<Msg> {
    match take(s) {
        b"[" => {
            let mut v = vec![];
            loop {
                match peek(s) {
                    b"]" => {
                        take(s);
                        break;
                    }
                    b"," => {
                        take(s);
                    }
                    _ => v.push(parse(s)?),
                }
            }
            Ok(Msg::Vec(v))
        }
        t => Ok(Msg::Int(std::str::from_utf8(t)?.parse()?)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Msg {
    Int(u32),
    Vec(Vec<Msg>),
}
impl<'a> TryFrom<&'a [u8]> for Msg {
    type Error = anyhow::Error;
    fn try_from(mut s: &'a [u8]) -> anyhow::Result<Msg> {
        let res = parse(&mut s)?;
        if !s.is_empty() {
            anyhow::bail!("Unreaded: {:?}", s);
        }
        Ok(res)
    }
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

    let first = Msg::try_from(b"[[2]]".as_slice())?;
    let second = Msg::try_from(b"[[6]]".as_slice())?;
    let mut msgs = vec![first.clone(), second.clone()];

    let mut iter = io::BufReader::new(std::fs::File::open("data/input13.txt")?)
        .lines()
        .map(|s| s.map(String::into_bytes));
    for i in 1.. {
        let Some(l) = iter.next() else { break };
        let Some(r) = iter.next() else { anyhow::bail!("Unexpected end of stream") };

        let l = Msg::try_from(l?.as_slice())?;
        let r = Msg::try_from(r?.as_slice())?;
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
