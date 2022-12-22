use std::io::{self, BufRead};

type Step = fn(&[Vec<u8>], &mut (usize, usize), &mut u8);

fn step_plane(map: &[Vec<u8>], coord: &mut (usize, usize), dir: &mut u8) {
    loop {
        let len0 = map.len();
        let len1 = map[coord.0].len();
        *coord = match dir {
            0 => (coord.0, (coord.1 + 1) % len1),
            1 => ((coord.0 + 1) % len0, coord.1),
            2 => (coord.0, (coord.1 + len1 - 1) % len1),
            3 => ((coord.0 + len0 - 1) % len0, coord.1),
            _ => panic!("bad dir {}", dir),
        };
        if map
            .get(coord.0)
            .and_then(|m| m.get(coord.1))
            .unwrap_or(&b' ')
            != &b' '
        {
            break;
        }
    }
}

fn step_cube(map: &[Vec<u8>], coord: &mut (usize, usize), dir: &mut u8) {
    match (*dir, *coord) {
        (3, (0, 50..=99)) => {
            *dir = 0;
            coord.0 = coord.1 + 100;
            coord.1 = 0;
        }
        (2, (100..=149, 0)) => {
            *dir = 0;
            coord.0 = 149 - coord.0;
            coord.1 = 50;
        }
        (2, (150..=199, 0)) => {
            *dir = 1;
            coord.1 = coord.0 - 100;
            coord.0 = 0;
        }
        (1, (199, 0..=49)) => {
            *dir = 1;
            coord.1 = coord.1 + 100;
            coord.0 = 0;
        }
        (0, (50..=99, 99)) => {
            *dir = 3;
            coord.1 = coord.0 + 50;
            coord.0 = 49;
        }
        (1, (49, 100..=149)) => {
            *dir = 2;
            coord.0 = coord.1 - 50;
            coord.1 = 99;
        }
        (0, (100..=149, 99)) => {
            *dir = 2;
            coord.0 = 149 - coord.0;
            coord.1 = 149;
        }
        (0, (0..=49, 149)) => {
            *dir = 2;
            coord.0 = 149 - coord.0;
            coord.1 = 99;
        }
        (2, (50..=99, 50)) => {
            *dir = 1;
            coord.1 = coord.0 - 50;
            coord.0 = 100;
        }
        (2, (0..=49, 50)) => {
            *dir = 0;
            coord.0 = 149 - coord.0;
            coord.1 = 0;
        }
        (1, (149, 50..=99)) => {
            *dir = 2;
            coord.0 = coord.1 + 100;
            coord.1 = 49;
        }
        (3, (0, 100..=149)) => {
            coord.1 = coord.1 - 100;
            coord.0 = 199;
        }
        (0, (150..=199, 49)) => {
            *dir = 3;
            coord.1 = coord.0 - 100;
            coord.0 = 149;
        }
        (3, (100, 0..=49)) => {
            *dir = 0;
            coord.0 = coord.1 + 50;
            coord.1 = 50;
        }
        _ => {
            *coord = match *dir {
                0 => (coord.0, coord.1 + 1),
                1 => (coord.0 + 1, coord.1),
                2 => (coord.0, coord.1 - 1),
                3 => (coord.0 - 1, coord.1),
                _ => panic!("bad dir {}", dir),
            };
        }
    }
    let point = map.get(coord.0).and_then(|m| m.get(coord.1)).copied();
    if ![Some(b'.'), Some(b'#')].contains(&point) {
        panic!("unhandled {} {:?}", dir, coord);
    }
}
fn advance(
    map: &[Vec<u8>],
    step: Step,
    mut coord: (usize, usize),
    mut dir: u8,
) -> Option<(u8, (usize, usize))> {
    step(map, &mut coord, &mut dir);
    (map[coord.0][coord.1] == b'.').then_some((dir, coord))
}

fn run(map: &[Vec<u8>], mut instrs: &str, step: Step) -> anyhow::Result<usize> {
    let mut coord = (0, map[0].iter().position(|&c| c == b'.').unwrap());
    let mut dir = 0_u8;
    loop {
        let pos = instrs.find(['R', 'L']).unwrap_or(instrs.len());
        let (nb, q) = instrs.split_at(pos);
        instrs = q;
        let nb = nb.parse::<u32>()?;
        for _ in 0..nb {
            let Some(new) = advance(&map, step, coord, dir) else { break };
            dir = new.0;
            coord = new.1;
        }
        if instrs.is_empty() {
            break;
        }
        let (rotate, q) = instrs.split_at(1);
        instrs = q;
        match rotate {
            "R" => dir = (dir + 1) % 4,
            "L" => dir = (dir + 3) % 4,
            _ => anyhow::bail!("bad rotation {}", rotate),
        }
    }
    Ok((coord.0 + 1) * 1000 + (coord.1 + 1) * 4 + dir as usize)
}

fn main() -> anyhow::Result<()> {
    let mut map = io::BufReader::new(std::fs::File::open("data/input22.txt")?)
        .lines()
        .map(|l| Ok(l?.into_bytes()))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let instrs = String::from_utf8(map.pop().ok_or_else(|| anyhow::anyhow!("no instruction"))?)?;
    let Some(true) = map.pop().map(|l| l.is_empty()) else { anyhow::bail!("no empty line") };

    println!("Part1: {}", run(&map, &instrs, step_plane)?);
    println!("Part2: {}", run(&map, &instrs, step_cube)?);

    Ok(())
}
