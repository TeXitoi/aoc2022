use std::collections::HashSet;
use std::io::{self, BufRead};

type Coord = (usize, usize);

fn step_blizzards(map: &[Vec<bool>], blizzards: &mut [(u8, Coord)]) {
    for (d, coord) in blizzards {
        match *d {
            b'<' => {
                coord.1 -= 1;
                if coord.1 == 0 {
                    coord.1 = map[coord.0].len() - 2;
                }
            }
            b'>' => {
                coord.1 += 1;
                if coord.1 == map[coord.0].len() - 1 {
                    coord.1 = 1;
                }
            }
            b'v' => {
                coord.0 += 1;
                if coord.0 == map.len() - 2 {
                    coord.0 = 2;
                }
            }
            b'^' => {
                coord.0 -= 1;
                if coord.0 == 1 {
                    coord.0 = map.len() - 3;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn step_me(c: Coord, map: &[Vec<bool>]) -> impl Iterator<Item = Coord> + '_ {
    [
        (c.0, c.1),
        (c.0 + 1, c.1),
        (c.0 - 1, c.1),
        (c.0, c.1 + 1),
        (c.0, c.1 - 1),
    ]
    .into_iter()
    .filter(move |c| map[c.0][c.1])
}

fn search(
    map: &Vec<Vec<bool>>,
    blizzards: &mut [(u8, Coord)],
    origin: Coord,
    destination: Coord,
) -> u32 {
    let mut positions = HashSet::from([origin]);
    let mut cur_map = map.clone();
    for t in 1.. {
        step_blizzards(map, blizzards);
        cur_map.clone_from(map);
        for (_, c) in blizzards.iter() {
            cur_map[c.0][c.1] = false;
        }
        positions = positions
            .iter()
            .flat_map(|&p| step_me(p, &cur_map))
            .collect();
        if positions.contains(&destination) {
            return t;
        }
    }
    unreachable!()
}

fn main() -> anyhow::Result<()> {
    let mut map = vec![];
    let mut blizzards = vec![];
    for (i, l) in io::BufReader::new(std::fs::File::open("data/input24.txt")?)
        .lines()
        .enumerate()
    {
        let l = l?.into_bytes();
        map.push(l.iter().map(|&c| c != b'#').collect::<Vec<_>>());
        for (j, c) in l.iter().enumerate() {
            if [b'>', b'v', b'<', b'^'].contains(c) {
                blizzards.push((*c, (i + 1, j)));
            }
        }
    }
    map.insert(0, map[0].iter().map(|_| false).collect());
    map.push(map[0].clone());
    let start = (1, map[1].iter().position(|&p| p).unwrap());
    let goal = (
        map.len() - 2,
        map[map.len() - 2].iter().position(|&p| p).unwrap(),
    );

    let first = search(&map, &mut blizzards, start, goal);
    println!("Part1: {}", first);
    let get_back = search(&map, &mut blizzards, goal, start);
    let and_then = search(&map, &mut blizzards, start, goal);
    println!("Part2: {}", first + get_back + and_then);

    Ok(())
}
