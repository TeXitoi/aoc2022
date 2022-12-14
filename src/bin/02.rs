use anyhow::bail;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}
use Move::*;

impl TryFrom<u8> for Move {
    type Error = anyhow::Error;
    fn try_from(b: u8) -> anyhow::Result<Self> {
        match b {
            b'A' | b'X' => Ok(Rock),
            b'B' | b'Y' => Ok(Paper),
            b'C' | b'Z' => Ok(Scissor),
            _ => bail!("Bad char: {}", b as char),
        }
    }
}
impl Move {
    fn play_score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }
    fn winning_score(self, us: Self) -> u32 {
        match (self, us) {
            (a, b) if a == b => 3,
            (Rock, Paper) | (Paper, Scissor) | (Scissor, Rock) => 6,
            _ => 0,
        }
    }
    fn score(self, us: Self) -> u32 {
        us.play_score() + self.winning_score(us)
    }
    fn move_for(self, res: RoundResult) -> Self {
        match (self, res) {
            (_, Draw) => self,
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissor,
            (Paper, Win) => Scissor,
            (Paper, Lose) => Rock,
            (Scissor, Win) => Rock,
            (Scissor, Lose) => Paper,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}
use RoundResult::*;

impl TryFrom<u8> for RoundResult {
    type Error = anyhow::Error;
    fn try_from(b: u8) -> anyhow::Result<Self> {
        match b {
            b'X' => Ok(Lose),
            b'Y' => Ok(Draw),
            b'Z' => Ok(Win),
            _ => bail!("Bad char: {}", b as char),
        }
    }
}

fn part1() -> anyhow::Result<()> {
    let mut score = 0;
    for l in read()? {
        let (them, us) = l?;
        let (them, us): (Move, Move) = (them.try_into()?, us.try_into()?);
        score += them.score(us);
    }
    println!("Part1: {}", score);
    Ok(())
}

fn part2() -> anyhow::Result<()> {
    let mut score = 0;
    for l in read()? {
        let (them, res) = l?;
        let (them, res): (Move, RoundResult) = (them.try_into()?, res.try_into()?);
        let us = them.move_for(res);
        score += them.score(us);
    }
    println!("Part2: {}", score);
    Ok(())
}

fn read() -> anyhow::Result<impl Iterator<Item = anyhow::Result<(u8, u8)>>> {
    let f = std::io::BufReader::new(std::fs::File::open("data/input2.txt")?);
    Ok(f.lines().map(|l| {
        let l = l?;
        let &[right, _, left] = l.as_bytes() else { bail!("bad line {}", l) };
        Ok((right, left))
    }))
}

fn main() -> anyhow::Result<()> {
    part1()?;
    part2()
}
