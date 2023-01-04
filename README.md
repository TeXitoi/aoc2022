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

# Days

In this section, I make a few remarks on the different days. I will
suppose you have already read the instructions on the official site.

## [Day 1](src/bin/01.rs)

This implementation use a
[BinaryHeap](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html).

## [Day 2](src/bin/02.rs)

This implementation use a lot rust "plain enum", and implement the
[TryFrom](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html)
on them.
