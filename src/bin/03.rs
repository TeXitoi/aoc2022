use std::collections::HashSet;
use std::io::BufRead;

fn priority(&c: &u8) -> u32 {
    (if c < b'a' {
        c - b'A' + 27
    } else {
        c - b'a' + 1
    }) as u32
}

fn part1() -> anyhow::Result<()> {
    let mut score = 0;
    for l in read()? {
        let l = l?;
        let (l, r) = l.split_at(l.len() / 2);
        let l: HashSet<_> = l.iter().copied().collect();
        let r = r.iter().copied().collect();
        score += l.intersection(&r).map(priority).sum::<u32>();
    }
    println!("Part1: {}", score);
    Ok(())
}

fn get_three(
    mut iter: impl Iterator<Item = anyhow::Result<Vec<u8>>>,
) -> anyhow::Result<Option<[HashSet<u8>; 3]>> {
    let Some(one) = iter.next().transpose()? else { return Ok(None) };
    let Some(two) = iter.next().transpose()? else { return Ok(None) };
    let Some(three) = iter.next().transpose()? else { return Ok(None) };
    Ok(Some([
        one.into_iter().collect(),
        two.into_iter().collect(),
        three.into_iter().collect(),
    ]))
}

fn part2() -> anyhow::Result<()> {
    let mut iter = read()?;
    let mut score = 0;
    while let Some([one, two, three]) = get_three(&mut iter)? {
        score += one
            .intersection(&two)
            .filter(|c| three.contains(c))
            .map(priority)
            .sum::<u32>();
    }
    println!("Part2: {}", score);
    Ok(())
}

fn read() -> anyhow::Result<impl Iterator<Item = anyhow::Result<Vec<u8>>>> {
    let f = std::io::BufReader::new(std::fs::File::open("data/input3.txt")?);
    Ok(f.lines().map(|l| Ok(l.map(String::into_bytes)?)))
}

fn main() -> anyhow::Result<()> {
    part1()?;
    part2()
}
