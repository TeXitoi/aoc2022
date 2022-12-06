use std::io::Read;

fn all_unique(buf: &[u8]) -> bool {
    buf.iter()
        .enumerate()
        .all(|(i, c1)| buf[..i].iter().all(|c2| c1 != c2))
}

fn find_first(buf: &[u8], size: usize) -> anyhow::Result<usize> {
    for (i, c) in buf.windows(size).enumerate() {
        if all_unique(c) {
            return Ok(i + size);
        }
    }
    anyhow::bail!("not found")
}

fn main() -> anyhow::Result<()> {
    assert_eq!(5, find_first(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 4)?);
    assert_eq!(6, find_first(b"nppdvjthqldpwncqszvftbrmjlhg", 4)?);
    assert_eq!(10, find_first(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)?);
    assert_eq!(11, find_first(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)?);

    let mut buf = vec![];
    std::fs::File::open("data/input6.txt")?.read_to_end(&mut buf)?;
    println!("part1: {}", find_first(&buf, 4)?);
    println!("part2: {}", find_first(&buf, 14)?);

    Ok(())
}
