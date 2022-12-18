use std::io::Read;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    nb_fallen: usize,
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
            nb_fallen: 0,
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

            self.nb_fallen += 1;

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
    fn state(&self) -> (&[u16], Rock, usize) {
        let len = self.cave.len();
        let cave = if len < 10 {
            &self.cave
        } else {
            &self.cave[len - 10..]
        };
        (cave, self.cur_rock, len - self.cur_rock_height)
    }
}

fn main() -> anyhow::Result<()> {
    let mut buf = vec![];
    std::fs::File::open("data/input17.txt")?.read_to_end(&mut buf)?;

    let mut c = Context::new();
    for &jet in buf.iter().cycle() {
        c.step(jet);
        if c.nb_fallen >= 2022 {
            break;
        }
    }
    println!("part1: {}", c.height(),);

    let mut slow = Context::new();
    let mut fast = Context::new();
    loop {
        for &jet in &buf {
            slow.step(jet);
            fast.step(jet);
        }
        for &jet in &buf {
            fast.step(jet);
        }
        if slow.state() == fast.state() {
            break;
        }
    }

    let nb_fallen_bulk = fast.nb_fallen - slow.nb_fallen;
    let height_bulk = fast.height() - slow.height();

    let mut wanted: usize = 1000000000000;
    wanted -= slow.nb_fallen;
    let mut height = slow.height();
    height += (wanted / nb_fallen_bulk) * height_bulk;
    wanted %= nb_fallen_bulk;

    slow.nb_fallen = 0;
    let already_height = slow.height();
    for &jet in buf.iter().cycle() {
        slow.step(jet);
        if slow.nb_fallen >= wanted {
            break;
        }
    }
    height += slow.height() - already_height;
    println!("part2: {}", height);

    Ok(())
}
