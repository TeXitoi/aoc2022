use std::io::BufRead;

fn visit(
    lines: &mut impl Iterator<Item = std::io::Result<String>>,
    f: &mut impl FnMut(u32),
) -> anyhow::Result<u32> {
    let mut cur_size = 0;
    loop {
        let Some(line) = lines.next() else { break };
        let line = line?;
        match line.split(' ').collect::<Vec<_>>().as_slice() {
            &["$", "cd", "/"] => (),
            &["$", "cd", ".."] => break,
            &["$", "cd", _] => cur_size += visit(lines, f)?,
            &["$", "ls"] => (),
            &["dir", _] => (),
            &[size, _] => cur_size += size.parse::<u32>()?,
            _ => anyhow::bail!("unsupported command {:?}", line),
        }
    }
    f(cur_size);
    Ok(cur_size)
}

fn read() -> anyhow::Result<impl Iterator<Item = std::io::Result<String>>> {
    Ok(std::io::BufReader::new(std::fs::File::open("data/input7.txt")?).lines())
}

fn main() -> anyhow::Result<()> {
    let mut sum_small_dirs = 0;
    let total_size = visit(&mut read()?, &mut |cur_size| {
        if cur_size <= 100000 {
            sum_small_dirs += cur_size;
        }
    })?;
    println!("total size: {}", total_size);
    println!("part1: {}", sum_small_dirs);

    let target = 30000000 - (70000000 - total_size);
    println!("target: {}", target);

    let mut to_remove_size = total_size;
    visit(&mut read()?, &mut |cur_size| {
        if cur_size >= target && cur_size < to_remove_size {
            to_remove_size = cur_size;
        }
    })?;
    println!("part2: {}", to_remove_size);

    Ok(())
}
