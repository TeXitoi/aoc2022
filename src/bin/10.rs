use std::io::{self, BufRead};

fn draw(cycle: i32, x: i32) {
    let pos = (cycle - 1) % 40;
    if (pos - x).abs() <= 1 {
        print!("#");
    } else {
        print!(" ");
    }
    if pos == 39 {
        println!();
    }
}

fn strength(cycle: i32, x: i32) -> i32 {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        cycle * x
    } else {
        0
    }
}

fn main() -> anyhow::Result<()> {
    let mut strengths = 0;
    let mut cycle = 1;
    let mut x = 1;
    for l in io::BufReader::new(std::fs::File::open("data/input10.txt")?).lines() {
        let l = l?;
        strengths += strength(cycle, x);
        draw(cycle, x);
        cycle += 1;
        match *l.split(' ').collect::<Vec<_>>() {
            ["noop"] => {}
            ["addx", nb] => {
                strengths += strength(cycle, x);
                draw(cycle, x);
                x += nb.parse::<i32>()?;
                cycle += 1;
            }
            _ => anyhow::bail!("bad line {:?}", l),
        }
    }
    println!("Part1: {}", strengths);

    Ok(())
}
