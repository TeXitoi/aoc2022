use std::io::{self, BufRead};

fn decode(s: &str) -> anyhow::Result<u64> {
    match s.chars().next() {
        None => anyhow::bail!("empty string"),
        Some(s) if !('0'..='2').contains(&s) => anyhow::bail!("unsupported first char {:?}", s),
        _ => (),
    }
    let mut res = 0;
    for c in s.chars() {
        res *= 5;
        match c {
            '0' => (),
            '1' => res += 1,
            '2' => res += 2,
            '-' => res -= 1,
            '=' => res -= 2,
            _ => anyhow::bail!("unsupported char {:?}", c),
        }
    }
    Ok(res)
}

fn encode(mut i: u64) -> String {
    if i == 0 {
        return "0".into();
    }
    let mut s = String::new();
    while i != 0 {
        match i % 5 {
            0 => s.push('0'),
            1 => s.push('1'),
            2 => s.push('2'),
            3 => {
                s.push('=');
                i += 5;
            }
            4 => {
                s.push('-');
                i += 5;
            }
            _ => unreachable!(),
        }
        i /= 5;
    }
    s.chars().rev().collect()
}

fn main() -> anyhow::Result<()> {
    let mut total = 0;
    for l in io::BufReader::new(std::fs::File::open("data/input25.txt")?).lines() {
        total += decode(&l?)?;
    }
    println!("Part1: {}", encode(total));

    Ok(())
}
