use std::io::BufRead;

fn read() -> anyhow::Result<impl Iterator<Item = anyhow::Result<[u32; 4]>>> {
    let f = std::io::BufReader::new(std::fs::File::open("data/input4.txt")?);
    Ok(f.lines().map(|l| {
        let v: Vec<u32> = l?
            .split(&['-', ','])
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(<[_; 4]>::try_from(v.as_slice())?)
    }))
}

fn is_included([b1, e1, b2, e2]: [u32; 4]) -> bool {
    b1 <= b2 && e2 <= e1 || b2 <= b1 && e1 <= e2
}

fn overlap([b1, e1, b2, e2]: [u32; 4]) -> bool {
    !(e1 < b2 || e2 < b1)
}

fn run(f: fn([u32; 4]) -> bool) -> anyhow::Result<u32> {
    let mut score = 0;
    for entry in read()? {
        let entry = entry?;
        score += f(entry) as u32;
    }
    Ok(score)
}

fn main() -> anyhow::Result<()> {
    println!("Part1: {}", run(is_included)?);
    println!("Part2: {}", run(overlap)?);
    Ok(())
}
