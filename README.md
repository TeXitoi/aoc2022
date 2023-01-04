# Advent of Code 2022

This repository contains my solutions for [Advent of Code
2022](https://adventofcode.com/2022/).

Goal is readable, simple and mostly clean.

Each day is solved in it's dedicated file in the [src/bin](src/bin/)
directory.

Only a few well-known dependencies are used:
* [anyhow](https://crates.io/crates/anyhow) for easy and simple error
  handling, in all the files.
* [regex](https://crates.io/crates/regex) is used in a few files for
  easy parsing.
* [serde](https://crates.io/crates/serde) and
  [serde\_json](https://crates.io/crates/serde_json) are used in
  [day 13](src/bin/13.rs) for easy parsing.

# About my writing of these files

I am an experienced rust developer. I use rust since 2014 (so before
rust 1.0). You may know me for
[structopt](https://github.com/TeXitoi/structopt) or
[keyberon](https://github.com/TeXitoi/keyberon). I like to use
iterators, the `?` operator and prefer (a bit too much) short names to
comments.

I have solved these problems by doing some "dirty" things (as
`.clone()` abuse, copy and paste, unreadable mess, damn slow algorithm
running during lunch). Then I have cleaned them, and sometime improved
them. They all run in less than 2 seconds in release on my computer.

All these programs should solve any problem from the official site,
except [day 22](src/bin/22.rs) (the cube folding is hardcoded for my
instance).

# Days

In this section, I make a few remarks on the different days. I will
suppose you have already read the instructions on the official site.

## [Day 1](src/bin/01.rs)

This implementation use a
[BinaryHeap](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html).

## [Day 2](src/bin/02.rs)

This implementation use a lot rust "plain enum", and implement the
[TryFrom](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html)
on them. It also externalize the preprocessing of the input in a
function returning an (somethat) `impl Iterator`.

## [Day 3](src/bin/03.rs)

This implementation use
[HashSets](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html)
and the `let else` new (at the time of writing) feature.
