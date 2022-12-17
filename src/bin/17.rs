use std::io::Read;

#[derive(Debug, Copy, Clone)]
struct Rock([u16; 4]);
impl Rock {
    fn manage_jet(&mut self, jet: u8, cave: &[u16]) {
        let mut new = *self;
        let shift = match jet {
            b'<' => |x| x << 1,
            b'>' => |x| x >> 1,
            _ => panic!("bad move {:?}", jet as char),
        };
        for l in &mut new.0 {
            *l = shift(*l);
        }
        if new.touch_something(cave) {
            return;
        }
        *self = new;
    }
    fn touch_something(&self, cave: &[u16]) -> bool {
        cave.iter().zip(&self.0).any(|(c, r)| c & r != 0)
    }
    fn stabilize(&self, cave: &mut [u16]) {
        for (c, r) in cave.iter_mut().zip(&self.0) {
            *c |= *r;
        }
    }
}

static ROCKS: &[Rock] = &[
    Rock([
        0b0001_1110_0000_0000,
        0b0000_0000_0000_0000,
        0b0000_0000_0000_0000,
        0b0000_0000_0000_0000,
    ]),
    Rock([
        0b0000_1000_0000_0000,
        0b0001_1100_0000_0000,
        0b0000_1000_0000_0000,
        0b0000_0000_0000_0000,
    ]),
    Rock([
        0b0001_1100_0000_0000,
        0b0000_0100_0000_0000,
        0b0000_0100_0000_0000,
        0b0000_0000_0000_0000,
    ]),
    Rock([
        0b0001_0000_0000_0000,
        0b0001_0000_0000_0000,
        0b0001_0000_0000_0000,
        0b0001_0000_0000_0000,
    ]),
    Rock([
        0b0001_1000_0000_0000,
        0b0001_1000_0000_0000,
        0b0000_0000_0000_0000,
        0b0000_0000_0000_0000,
    ]),
];
const CAVE_LAYER: u16 = 0b1000_0000_1111_1111;

struct Context {
    rocks: std::iter::Cycle<std::iter::Copied<std::slice::Iter<'static, Rock>>>,
    cur_rock_height: usize,
    cur_rock: Rock,
    cave: Vec<u16>,
    nb_fallen_rocks: usize,
}
impl Context {
    fn new() -> Self {
        let mut rocks = ROCKS.iter().copied().cycle();
        let mut cave = vec![CAVE_LAYER; 8];
        cave[0] = 0b1111_1111_1111_1111;
        Self {
            cur_rock: rocks.next().unwrap(),
            rocks,
            cur_rock_height: 4,
            cave,
            nb_fallen_rocks: 0,
        }
    }
    fn step(&mut self, jet: u8) {
        self.cur_rock
            .manage_jet(jet, &self.cave[self.cur_rock_height..]);

        if self
            .cur_rock
            .touch_something(&self.cave[self.cur_rock_height - 1..])
        {
            self.cur_rock
                .stabilize(&mut self.cave[self.cur_rock_height..]);

            self.nb_fallen_rocks += 1;

            self.cur_rock = self.rocks.next().unwrap();
            let height = self
                .cave
                .iter()
                .position(|&c| c == CAVE_LAYER)
                .unwrap_or(self.cave.len());
            self.cur_rock_height = height + 3;
            self.cave.resize(self.cur_rock_height + 4, CAVE_LAYER);
        } else {
            self.cur_rock_height -= 1;
        }
    }
    fn height(&self) -> usize {
        self.cave[1..].iter().filter(|&&l| l != CAVE_LAYER).count()
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = vec![];
    std::fs::File::open("data/input17.txt")?.read_to_end(&mut buf)?;
    let buf_clone = buf.clone();
    for _ in 0..4 {
        buf.extend_from_slice(&buf_clone);
    }

    let mut c = Context::new();
    for jet in buf.iter().copied().cycle() {
        c.step(jet);
        if c.nb_fallen_rocks >= 2022 {
            break;
        }
    }
    println!("part1: {}", c.height(),);

    c = Context::new();
    let mut wanted: usize = 1000000000000;
    for jet in buf.iter().copied() {
        c.step(jet);
    }
    let first_nb_fallen_rocks = c.nb_fallen_rocks;
    let first_height = c.height();
    for jet in buf.iter().copied() {
        c.step(jet);
    }

    wanted -= c.nb_fallen_rocks;
    let mut height = c.height();
    let nb_fallen_bulk = c.nb_fallen_rocks - first_nb_fallen_rocks;
    let height_bulk = c.height() - first_height;
    let second_height = c.height();

    height += (wanted / nb_fallen_bulk) * height_bulk;
    wanted %= nb_fallen_bulk;
    c.nb_fallen_rocks = 0;
    for jet in buf.iter().copied().cycle() {
        c.step(jet);
        if c.nb_fallen_rocks >= wanted {
            break;
        }
    }

    height += c.height() - second_height;

    println!("part2: {}", height);

    Ok(())
}
