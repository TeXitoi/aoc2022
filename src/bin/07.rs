use std::io::{self, BufRead};

fn visit(
    lines: &mut (impl Iterator<Item = io::Result<String>> + std::iter::FusedIterator),
    f: &mut impl FnMut(u32),
) -> anyhow::Result<u32> {
    let mut cur_size = 0;
    loop {
        let Some(line) = lines.next() else { break };
        let line = line?;
        match *line.split(' ').collect::<Vec<_>>() {
            ["$", "cd", "/"] => (),
            ["$", "cd", ".."] => break,
            ["$", "cd", _] => cur_size += visit(lines, f)?,
            ["$", "ls"] => (),
            ["dir", _] => (),
            [size, _] => cur_size += size.parse::<u32>()?,
            _ => anyhow::bail!("unsupported command {:?}", line),
        }
    }
    f(cur_size);
    Ok(cur_size)
}

fn read() -> anyhow::Result<impl Iterator<Item = io::Result<String>> + std::iter::FusedIterator> {
    Ok(io::BufReader::new(std::fs::File::open("data/input7.txt")?)
        .lines()
        .fuse())
}

fn main() -> anyhow::Result<()> {
    let mut sum_small_dirs = 0;
    let total_size = visit(&mut read()?, &mut |cur_size| {
        if cur_size <= 100000 {
            sum_small_dirs += cur_size;
        }
    })?;
    println!("Part1: {}", sum_small_dirs);

    let target = 30000000 - (70000000 - total_size);
    let mut to_remove_size = total_size;
    visit(&mut read()?, &mut |cur_size| {
        if cur_size >= target && cur_size < to_remove_size {
            to_remove_size = cur_size;
        }
    })?;
    println!("Part2: {}", to_remove_size);

    Ok(())
}
